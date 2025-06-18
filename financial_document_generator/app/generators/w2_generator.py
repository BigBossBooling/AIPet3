import io
from typing import Dict, Any, Union, Optional, List # Updated imports
from .base_generator import BaseGenerator
from weasyprint import HTML, FontConfiguration # WeasyPrint imports

class W2Generator(BaseGenerator):
    """
    Generates a W-2 form PDF using a Jinja2 HTML template and WeasyPrint.
    Inherits from BaseGenerator.
    """
    def __init__(self, data: dict):
        """
        Initializes W2Generator with data.
        Args:
            data (dict): A dictionary containing W-2 form fields.
                         It can be a flat dictionary (e.g., from a form or simple API)
                         or already contain structured items like 'box_12_items'.
        """
        super().__init__(data) # self.data is now populated
        self._prepare_template_data() # Process and structure data for the template

    def _get_bool_from_data(self, key: str, default: bool = False) -> bool:
        """Helper to convert various truthy/falsy inputs to boolean."""
        val = self.data.get(key)
        if isinstance(val, str):
            return val.lower() in ['true', '1', 't', 'y', 'yes']
        return bool(val) if val is not None else default

    def _prepare_template_data(self) -> None:
        """
        Prepares and structures self.data for the w2_template.html.
        This involves converting flat data for Box 12, 14, state, and local taxes
        into lists of dictionaries expected by the template.
        It also ensures all expected top-level fields from the old __init__ are present in self.data,
        defaulting them if necessary (though BaseGenerator's self.data already holds them).
        """

        # Ensure base numeric fields are floats (original __init__ did this)
        numeric_fields = [
            "wages_tips_other_compensation", "federal_income_tax_withheld",
            "social_security_wages", "medicare_wages_and_tips",
            "social_security_tax_withheld", "medicare_tax_withheld",
            "social_security_tips", "allocated_tips", "dependent_care_benefits",
            "nonqualified_plans", "other_amount_code_d", # Assuming this is a specific Box 14 item
            "state_wages_tips_etc", "state_income_tax", # For the first state
            "state_wages_tips_etc_2", "state_income_tax_2", # For a second state
            "local_wages_tips_etc", "local_income_tax", # For the first locality
            "local_wages_tips_etc_2", "local_income_tax_2"  # For a second locality
        ]
        for field in numeric_fields:
            self.data[field] = float(self.data.get(field, 0.0))

        # Ensure string fields default to empty string if not present
        string_fields = [
            "employee_name", "employee_ssn", "employee_address", "employee_city_state_zip",
            "employer_name", "employer_address", "employer_city_state_zip", "employer_ein",
            "control_number", "state_employer_state_id_no", "locality_name", # For first state/locality
            "state_code_1", "state_employer_state_id_no_1", # For explicit first state
            "state_code_2", "state_employer_state_id_no_2", # For explicit second state
            "locality_name_1", "locality_name_2", # For explicit localities
            "other_description_code_d" # Assuming this is a specific Box 14 item
        ]
        for field in string_fields:
            self.data[field] = str(self.data.get(field, '')).strip()

        # Box 12 items
        self.data['box_12_items'] = []
        for char_code in ['a', 'b', 'c', 'd']:
            code = self.data.get(f'box_12{char_code}_code', '').strip()
            amount_val = self.data.get(f'box_12{char_code}_amount')
            if code: # Only add if code is present
                amount = 0.0
                if amount_val is not None:
                    try:
                        amount = float(amount_val)
                    except (ValueError, TypeError):
                        amount = 0.0 # Or log warning
                self.data['box_12_items'].append({'code': code, 'amount': amount})

        # Box 13 checkboxes
        self.data['statutory_employee'] = self._get_bool_from_data('statutory_employee')
        self.data['retirement_plan'] = self._get_bool_from_data('retirement_plan')
        self.data['third_party_sick_pay'] = self._get_bool_from_data('third_party_sick_pay')

        # Box 14 "Other" items
        self.data['box_14_items'] = []
        # Example for a primary "other" item, can be expanded for more from flat data
        if self.data.get('other_description_code_d') or self.data.get('other_amount_code_d', 0.0) != 0.0:
             self.data['box_14_items'].append({
                 'description': self.data.get('other_description_code_d', ''),
                 'amount': self.data.get('other_amount_code_d', 0.0)
             })
        # Allow passing 'box_14_items' directly as a list in input data for more flexibility
        if 'box_14_items_input' in self.data and isinstance(self.data['box_14_items_input'], list):
            for item in self.data['box_14_items_input']:
                if isinstance(item, dict) and 'description' in item and 'amount' in item:
                    try:
                        item_amount = float(item['amount'])
                        self.data['box_14_items'].append({'description': str(item['description']), 'amount': item_amount})
                    except (ValueError, TypeError): pass # Skip malformed item


        # State Tax Items (up to 2 states)
        self.data['state_tax_items'] = []
        # First state (using primary fields or specific _1 fields)
        state_code_1 = self.data.get('state_code_1', self.data.get('state', '')) # 'state' for backward compatibility
        emp_id_1 = self.data.get('state_employer_state_id_no_1', self.data.get('state_employer_state_id_no', ''))
        wages_1 = self.data.get('state_wages_tips_etc_1', self.data.get('state_wages_tips_etc', 0.0))
        tax_1 = self.data.get('state_income_tax_1', self.data.get('state_income_tax', 0.0))
        if state_code_1 or emp_id_1 or wages_1 > 0 or tax_1 > 0:
            self.data['state_tax_items'].append({
                'state_code': state_code_1, 'employer_state_id': emp_id_1,
                'state_wages': wages_1, 'state_income_tax': tax_1
            })
        # Second state (using specific _2 fields)
        state_code_2 = self.data.get('state_code_2', '')
        emp_id_2 = self.data.get('state_employer_state_id_no_2', '')
        wages_2 = self.data.get('state_wages_tips_etc_2', 0.0)
        tax_2 = self.data.get('state_income_tax_2', 0.0)
        if state_code_2 or emp_id_2 or wages_2 > 0 or tax_2 > 0:
             self.data['state_tax_items'].append({
                'state_code': state_code_2, 'employer_state_id': emp_id_2,
                'state_wages': wages_2, 'state_income_tax': tax_2
            })

        # Local Tax Items (up to 2 localities)
        self.data['local_tax_items'] = []
        # First locality
        loc_wages_1 = self.data.get('local_wages_tips_etc_1', self.data.get('local_wages_tips_etc', 0.0))
        loc_tax_1 = self.data.get('local_income_tax_1', self.data.get('local_income_tax', 0.0))
        loc_name_1 = self.data.get('locality_name_1', self.data.get('locality_name', ''))
        if loc_wages_1 > 0 or loc_tax_1 > 0 or loc_name_1:
            self.data['local_tax_items'].append({
                'local_wages': loc_wages_1, 'local_income_tax': loc_tax_1, 'locality_name': loc_name_1
            })
        # Second locality
        loc_wages_2 = self.data.get('local_wages_tips_etc_2', 0.0)
        loc_tax_2 = self.data.get('local_income_tax_2', 0.0)
        loc_name_2 = self.data.get('locality_name_2', '')
        if loc_wages_2 > 0 or loc_tax_2 > 0 or loc_name_2:
             self.data['local_tax_items'].append({
                'local_wages': loc_wages_2, 'local_income_tax': loc_tax_2, 'locality_name': loc_name_2
            })

        # Ensure all keys expected by the template are present, even if empty/default
        for key in ['void_checkbox', 'employer_address', 'employer_city_state_zip',
                    'employee_address', 'employee_city_state_zip']:
            if key not in self.data: self.data[key] = '' # Or appropriate default (False for checkbox)


    def generate_pdf(self, output_path_or_buffer: Optional[Union[str, io.BytesIO]] = None) -> Optional[Union[bool, bytes]]:
        try:
            # self.data should already be prepared by __init__ -> _prepare_template_data
            html_string = self._render_template('w2_template.html', self.data)

            font_config = FontConfiguration()
            html_doc = HTML(string=html_string, base_url=self._get_project_root())

            is_path = isinstance(output_path_or_buffer, str)
            is_buffer = isinstance(output_path_or_buffer, io.BytesIO)

            if is_path:
                html_doc.write_pdf(output_path_or_buffer, font_config=font_config)
                return True
            elif is_buffer:
                # WeasyPrint's write_pdf can write to a buffer if target is provided
                # but it returns None in that case. To be consistent for buffer writes,
                # we get bytes and write them. Or, rely on caller to handle buffer.
                # For now, let's make it similar to path: write and return True.
                # The prompt had: output_path_or_buffer.write(pdf_bytes)
                # So, first get bytes, then write.
                pdf_bytes_val = html_doc.write_pdf(font_config=font_config)
                if pdf_bytes_val:
                    output_path_or_buffer.write(pdf_bytes_val)
                    return True
                return False # Error if no bytes
            else: # None, return bytes
                pdf_bytes = html_doc.write_pdf(font_config=font_config)
                return pdf_bytes
        except Exception as e:
            print(f"Error generating W2 PDF with WeasyPrint: {e}")
            # import traceback
            # traceback.print_exc()
            return None if output_path_or_buffer is None else False

    # to_dict and from_dict might need review if self.data is the primary source
    # and attributes are mainly for convenience or internal use.
    # For now, keeping them as they were, but they reflect the flat structure mostly.
    def to_dict(self) -> Dict[str, Any]:
        """Converts the W2Generator instance attributes to a dictionary."""
        # This should ideally return self.data after preparation, or a selection from it
        return self.data # Return the prepared data

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'W2Generator':
        """Creates a W2Generator object from a dictionary."""
        return cls(data)
