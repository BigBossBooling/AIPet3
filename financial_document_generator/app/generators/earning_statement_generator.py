import os
import io
from typing import Union, Optional, Dict, List, Any

from .base_generator import BaseGenerator
from weasyprint import HTML
from weasyprint.fonts import FontConfiguration

class EarningStatementGenerator(BaseGenerator):
    def __init__(self, data: Dict):
        """
        Initializes EarningStatementGenerator with earning statement data.
        Args:
            data (dict): A dictionary containing earning statement details.
                         Expected keys similar to CheckStubGenerator, plus 'corporate_notes'.
                         e.g., company_name, company_address, employee_name, employee_id,
                               pay_period_start, pay_period_end, pay_date,
                               earnings_items (list of dicts), earnings_total,
                               deduction_items (list of dicts), deductions_total,
                               net_pay, ytd_gross_earnings, ytd_deductions, ytd_net_pay,
                               corporate_notes.
                         The template also supports nested employer_info.name/address as fallback.
        """
        super().__init__(data)
        self._prepare_data()

    def _ensure_numeric_in_data(self, keys: List[str], default_value: float = 0.0) -> None:
        """Ensure specified keys in self.data are numeric (float)."""
        for key in keys:
            try:
                self.data[key] = float(self.data.get(key, default_value))
            except (ValueError, TypeError):
                self.data[key] = default_value

    def _ensure_list_of_dicts_in_data(self, list_key: str, item_amount_key: str = 'amount') -> None:
        """Ensure a key in self.data is a list of dicts with a numeric amount field."""
        items = self.data.get(list_key)
        if not isinstance(items, list):
            self.data[list_key] = []
            return

        processed_items = []
        for item in items:
            if isinstance(item, dict) and item_amount_key in item:
                try:
                    item[item_amount_key] = float(item.get(item_amount_key, 0.0))
                except (ValueError, TypeError):
                    item[item_amount_key] = 0.0
                processed_items.append(item)
        self.data[list_key] = processed_items

    def _prepare_data(self) -> None:
        """
        Prepares data for the earning statement.
        Ensures numeric fields are floats and item lists are correctly structured.
        Calculates totals from items if not provided.
        """
        numeric_totals = [
            'earnings_total', 'deductions_total', 'net_pay',
            'ytd_gross_earnings', 'ytd_deductions', 'ytd_net_pay'
        ]
        # Also ensure individual earnings/deductions (if used as fallback by template) are numeric
        # These keys are nested under 'earnings' and 'deductions' in the template for fallback
        if 'earnings' in self.data and isinstance(self.data['earnings'], dict):
            for k in self.data['earnings']: self.data['earnings'][k] = float(self.data['earnings'].get(k, 0.0))
        if 'deductions' in self.data and isinstance(self.data['deductions'], dict):
            for k in self.data['deductions']: self.data['deductions'][k] = float(self.data['deductions'].get(k, 0.0))

        self._ensure_numeric_in_data(numeric_totals)

        self._ensure_list_of_dicts_in_data('earnings_items')
        self._ensure_list_of_dicts_in_data('deduction_items')

        # Calculate totals from items if totals are zero or not provided explicitly
        if self.data.get('earnings_total', 0.0) == 0.0 and self.data.get('earnings_items'):
            self.data['earnings_total'] = sum(item.get('amount', 0.0) for item in self.data['earnings_items'])

        if self.data.get('deductions_total', 0.0) == 0.0 and self.data.get('deduction_items'):
            self.data['deductions_total'] = sum(item.get('amount', 0.0) for item in self.data['deduction_items'])

        # Calculate net_pay if not provided or zero
        if self.data.get('net_pay', 0.0) == 0.0:
            self.data['net_pay'] = self.data.get('earnings_total', 0.0) - self.data.get('deductions_total', 0.0)

        # Ensure default empty lists/strings for other optional fields for template rendering
        for key in ['earnings_items', 'deduction_items']:
            if key not in self.data:
                self.data[key] = []
        if 'corporate_notes' not in self.data:
            self.data['corporate_notes'] = ''


    def generate_pdf(self, output_path_or_buffer: Optional[Union[str, io.BytesIO]] = None) -> Optional[Union[bool, bytes]]:
        try:
            rendered_html = self._render_template('earning_statement.html', self.data)
            font_config = FontConfiguration()
            html_doc = HTML(string=rendered_html, base_url=self._get_project_root())

            is_path = isinstance(output_path_or_buffer, str)
            is_buffer = isinstance(output_path_or_buffer, io.BytesIO)

            if is_path:
                html_doc.write_pdf(output_path_or_buffer, font_config=font_config)
                return True
            elif is_buffer:
                html_doc.write_pdf(target=output_path_or_buffer, font_config=font_config)
                return True
            else: # None, return bytes
                pdf_bytes = html_doc.write_pdf(font_config=font_config)
                return pdf_bytes
        except Exception as e:
            print(f"Error generating earning statement PDF: {e}")
            # import traceback
            # traceback.print_exc()
            return None if output_path_or_buffer is None else False

# '''
# def generate_earning_statement(data):
#     # TODO: Implement earning statement generation logic
#     print(f"Generating earning statement with data: {data}")
#     return "path/to/generated/earning_statement.pdf" # Placeholder
# '''
