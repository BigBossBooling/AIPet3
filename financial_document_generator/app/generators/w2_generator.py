from typing import Dict, Any

class W2Generator:
    """
    Represents a W-2 form with its standard fields.
    """
    def __init__(self,
                 employee_name: str,
                 employee_ssn: str,
                 employer_name: str,
                 employer_ein: str,
                 wages_tips_other_compensation: float,
                 federal_income_tax_withheld: float,
                 social_security_wages: float = 0.0,
                 medicare_wages_and_tips: float = 0.0,
                 social_security_tax_withheld: float = 0.0,
                 medicare_tax_withheld: float = 0.0,
                 state_employer_state_id_no: str = "",
                 state_wages_tips_etc: float = 0.0,
                 state_income_tax: float = 0.0,
                 local_wages_tips_etc: float = 0.0,
                 local_income_tax: float = 0.0,
                 locality_name: str = ""):
        self.employee_name: str = employee_name
        self.employee_ssn: str = employee_ssn
        self.employer_name: str = employer_name
        self.employer_ein: str = employer_ein
        self.wages_tips_other_compensation: float = wages_tips_other_compensation
        self.federal_income_tax_withheld: float = federal_income_tax_withheld
        self.social_security_wages: float = social_security_wages
        self.medicare_wages_and_tips: float = medicare_wages_and_tips
        self.social_security_tax_withheld: float = social_security_tax_withheld
        self.medicare_tax_withheld: float = medicare_tax_withheld
        self.state_employer_state_id_no: str = state_employer_state_id_no
        self.state_wages_tips_etc: float = state_wages_tips_etc
        self.state_income_tax: float = state_income_tax
        self.local_wages_tips_etc: float = local_wages_tips_etc
        self.local_income_tax: float = local_income_tax
        self.locality_name: str = locality_name

    def to_dict(self) -> Dict[str, Any]:
        """Converts the W2Form object to a dictionary."""
        return self.__dict__

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'W2Generator':
        """Creates a W2Generator object from a dictionary."""
        return cls(
            employee_name=data.get("employee_name", ""),
            employee_ssn=data.get("employee_ssn", ""),
            employer_name=data.get("employer_name", ""),
            employer_ein=data.get("employer_ein", ""),
            wages_tips_other_compensation=data.get("wages_tips_other_compensation", 0.0),
            federal_income_tax_withheld=data.get("federal_income_tax_withheld", 0.0),
            social_security_wages=data.get("social_security_wages", 0.0),
            medicare_wages_and_tips=data.get("medicare_wages_and_tips", 0.0),
            social_security_tax_withheld=data.get("social_security_tax_withheld", 0.0),
            medicare_tax_withheld=data.get("medicare_tax_withheld", 0.0),
            state_employer_state_id_no=data.get("state_employer_state_id_no", ""),
            state_wages_tips_etc=data.get("state_wages_tips_etc", 0.0),
            state_income_tax=data.get("state_income_tax", 0.0),
            local_wages_tips_etc=data.get("local_wages_tips_etc", 0.0),
            local_income_tax=data.get("local_income_tax", 0.0),
            locality_name=data.get("locality_name", "")
        )

    def generate_pdf(self, output_path: str) -> None:
        """
        Generates a PDF representation of the W-2 form.
        """
        from reportlab.pdfgen import canvas
        from reportlab.lib.pagesizes import letter
        from reportlab.lib.units import inch

        try:
            c = canvas.Canvas(output_path, pagesize=letter)
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
            print(f"W-2 form generated successfully at {output_path}")

        except IOError as e:
            print(f"Error generating PDF: Could not write to file {output_path}. Error: {e}")
        except Exception as e:
            print(f"An unexpected error occurred during PDF generation: {e}")

from typing import Dict, Any
