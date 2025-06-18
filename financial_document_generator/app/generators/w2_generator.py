import io # Added for io.BytesIO
from typing import Dict, Any, Union, Optional # For type hinting
from .base_generator import BaseGenerator # Import BaseGenerator
from reportlab.pdfgen import canvas # Keep reportlab imports
from reportlab.lib.pagesizes import letter
from reportlab.lib.units import inch

class W2Generator(BaseGenerator):
    """
    Generates a W-2 form PDF using ReportLab.
    Inherits from BaseGenerator.
    """
    def __init__(self, data: dict):
        """
        Initializes W2Generator with data.

        Args:
            data (dict): A dictionary containing all necessary W-2 form fields.
        """
        super().__init__(data) # Call BaseGenerator's __init__

        # Populate instance attributes from data dictionary
        self.employee_name: str = self.data.get("employee_name", "")
        self.employee_ssn: str = self.data.get("employee_ssn", "")
        self.employer_name: str = self.data.get("employer_name", "")
        self.employer_ein: str = self.data.get("employer_ein", "")
        self.wages_tips_other_compensation: float = float(self.data.get("wages_tips_other_compensation", 0.0))
        self.federal_income_tax_withheld: float = float(self.data.get("federal_income_tax_withheld", 0.0))
        self.social_security_wages: float = float(self.data.get("social_security_wages", 0.0))
        self.medicare_wages_and_tips: float = float(self.data.get("medicare_wages_and_tips", 0.0))
        self.social_security_tax_withheld: float = float(self.data.get("social_security_tax_withheld", 0.0))
        self.medicare_tax_withheld: float = float(self.data.get("medicare_tax_withheld", 0.0))
        self.state_employer_state_id_no: str = self.data.get("state_employer_state_id_no", "")
        self.state_wages_tips_etc: float = float(self.data.get("state_wages_tips_etc", 0.0))
        self.state_income_tax: float = float(self.data.get("state_income_tax", 0.0))
        self.local_wages_tips_etc: float = float(self.data.get("local_wages_tips_etc", 0.0))
        self.local_income_tax: float = float(self.data.get("local_income_tax", 0.0))
        self.locality_name: str = self.data.get("locality_name", "")

    def to_dict(self) -> Dict[str, Any]:
        """Converts the W2Generator instance attributes to a dictionary."""
        # This could also leverage self.data if it's kept complete and attributes are just for convenience
        return {
            "employee_name": self.employee_name,
            "employee_ssn": self.employee_ssn,
            "employer_name": self.employer_name,
            "employer_ein": self.employer_ein,
            "wages_tips_other_compensation": self.wages_tips_other_compensation,
            "federal_income_tax_withheld": self.federal_income_tax_withheld,
            "social_security_wages": self.social_security_wages,
            "medicare_wages_and_tips": self.medicare_wages_and_tips,
            "social_security_tax_withheld": self.social_security_tax_withheld,
            "medicare_tax_withheld": self.medicare_tax_withheld,
            "state_employer_state_id_no": self.state_employer_state_id_no,
            "state_wages_tips_etc": self.state_wages_tips_etc,
            "state_income_tax": self.state_income_tax,
            "local_wages_tips_etc": self.local_wages_tips_etc,
            "local_income_tax": self.local_income_tax,
            "locality_name": self.locality_name,
        }

    # from_dict can be removed if initialization is always from a single data dict
    # Or it can be kept as a convenience constructor for the specific data structure it expects
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'W2Generator':
        """Creates a W2Generator object from a dictionary.
           This now primarily serves as a way to ensure data is in the expected structure.
        """
        return cls(data)

    def generate_pdf(self, output_path_or_buffer: Optional[Union[str, io.BytesIO]] = None) -> Optional[Union[bool, bytes]]:
        """
        Generates a PDF representation of the W-2 form using ReportLab.

        Args:
            output_path_or_buffer (str or io.BytesIO, optional):
                If a string, it's the path to save the PDF file.
                If an io.BytesIO object, the PDF is written to this buffer.
                If None, the PDF content is returned as bytes.

        Returns:
            bool: True if PDF was saved to a file path successfully, False on error.
            bytes: PDF content as bytes if output_path_or_buffer is None and successful.
            None: If an error occurred when trying to return bytes or save to a buffer.
                  (When saving to provided buffer, effectively returns None on success via buffer modification)
        """
        target_buffer = None
        is_path = isinstance(output_path_or_buffer, str)
        is_buffer = isinstance(output_path_or_buffer, io.BytesIO)

        if is_path:
            target_canvas_arg = output_path_or_buffer
        elif is_buffer:
            target_canvas_arg = output_path_or_buffer
        else: # None or other type
            target_buffer = io.BytesIO()
            target_canvas_arg = target_buffer

        try:
            c = canvas.Canvas(target_canvas_arg, pagesize=letter)
            width, height = letter  # (612, 792)

            # Set a title
            c.setFont("Helvetica-Bold", 16)
            c.drawString(0.5 * inch, height - 0.5 * inch, "Form W-2: Wage and Tax Statement")

            # Basic layout - simplified for brevity
            text_y_start = height - 1.5 * inch
            line_height = 0.25 * inch
            current_y = text_y_start
            left_margin_label = 0.5 * inch
            left_margin_value = 2.5 * inch

            c.setFont("Helvetica", 10)

            def draw_field(label: str, value: Any, y_pos: float):
                c.drawString(left_margin_label, y_pos, f"{label}:")
                c.drawString(left_margin_value, y_pos, str(value))
                return y_pos - line_height

            current_y = draw_field("Employee's SSN", self.employee_ssn, current_y)
            current_y = draw_field("Employer identification number (EIN)", self.employer_ein, current_y)

            current_y = draw_field("Employer's name, address, and ZIP code", self.employer_name, current_y)
            # For address, you'd typically have more fields or a formatted block.
            # c.drawString(left_margin_value, current_y, "Employer Address Line 1")
            # current_y -= 0.18 * inch
            # c.drawString(left_margin_value, current_y, "City, State, ZIP")
            # current_y -= line_height (adjust as needed)


            current_y = draw_field("Employee's first name and initial Last name", self.employee_name, current_y)
            # Similar for employee's address

            current_y = text_y_start - (4 * line_height) # Jump to a section for monetary values for simplicity

            # Box 1: Wages, tips, other compensation
            current_y = draw_field("1 Wages, tips, other compensation", f"${self.wages_tips_other_compensation:,.2f}", current_y)
            # Box 2: Federal income tax withheld
            current_y = draw_field("2 Federal income tax withheld", f"${self.federal_income_tax_withheld:,.2f}", current_y)
            # Box 3: Social security wages
            current_y = draw_field("3 Social security wages", f"${self.social_security_wages:,.2f}", current_y)
            # Box 4: Social security tax withheld
            current_y = draw_field("4 Social security tax withheld", f"${self.social_security_tax_withheld:,.2f}", current_y)
            # Box 5: Medicare wages and tips
            current_y = draw_field("5 Medicare wages and tips", f"${self.medicare_wages_and_tips:,.2f}", current_y)
            # Box 6: Medicare tax withheld
            current_y = draw_field("6 Medicare tax withheld", f"${self.medicare_tax_withheld:,.2f}", current_y)

            # State Information (Simplified)
            current_y -= line_height # Extra space before state info
            current_y = draw_field("15 State | Employer's state ID number", self.state_employer_state_id_no, current_y)
            current_y = draw_field("16 State wages, tips, etc.", f"${self.state_wages_tips_etc:,.2f}", current_y)
            current_y = draw_field("17 State income tax", f"${self.state_income_tax:,.2f}", current_y)

            # Local Information (Simplified)
            current_y -= line_height # Extra space before local info
            current_y = draw_field("18 Local wages, tips, etc.", f"${self.local_wages_tips_etc:,.2f}", current_y)
            current_y = draw_field("19 Local income tax", f"${self.local_income_tax:,.2f}", current_y)
            current_y = draw_field("20 Locality name", self.locality_name, current_y)

            c.save()

            if is_path:
                print(f"W-2 form generated successfully at {output_path_or_buffer}")
                return True
            elif is_buffer:
                # Data is written to the buffer, caller handles it.
                return None # Or True, to indicate success writing to buffer. Let's stick to None for now.
            else: # Was None, so target_buffer was used
                return target_buffer.getvalue()

        except IOError as e:
            print(f"IOError generating PDF: {e}")
            if is_path: return False
            return None # Error for buffer or None case
        except Exception as e:
            print(f"An unexpected error occurred during PDF generation: {e}")
            if is_path: return False
            return None # Error for buffer or None case
