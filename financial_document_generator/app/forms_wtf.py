from flask_wtf import FlaskForm
from wtforms import StringField, FloatField, BooleanField, SubmitField # Removed IntegerField as not used in W2FormWTF
from wtforms.validators import DataRequired, Length, Optional, NumberRange, Regexp # Removed FieldList, FormField for now

# Placeholder for Box 12 items - can be more complex if needed
# class Box12ItemForm(FlaskForm): # Not using this sub-form for initial simplicity with FieldList
#     code = StringField('Code', validators=[Optional(), Length(max=2)])
#     amount = FloatField('Amount', validators=[Optional(), NumberRange(min=0)])

class W2FormWTF(FlaskForm):
    # Employer Information
    employer_ein = StringField('Employer Identification Number (EIN)',
                               validators=[DataRequired(), Regexp(r'^\d{2}-?\d{7}$', message="EIN must be in XX-XXXXXXX format.")])
    employer_name = StringField('Employer Name', validators=[DataRequired(), Length(max=100)])
    employer_address = StringField('Employer Address (Street, PO Box)', validators=[DataRequired(), Length(max=100)]) # Corrected label from prompt
    employer_city_state_zip = StringField('Employer City, State, ZIP Code', validators=[DataRequired(), Length(max=100)])
    control_number = StringField('Control Number', validators=[Optional(), Length(max=50)])

    # Employee Information
    employee_ssn = StringField('Employee Social Security Number',
                               validators=[DataRequired(), Regexp(r'^\d{3}-?\d{2}-?\d{4}$', message="SSN must be in XXX-XX-XXXX format.")])
    employee_name = StringField('Employee Name (First, Last, Suffix)', validators=[DataRequired(), Length(max=100)])
    employee_address = StringField('Employee Address (Street)', validators=[DataRequired(), Length(max=100)])
    employee_city_state_zip = StringField('Employee City, State, ZIP Code', validators=[DataRequired(), Length(max=100)])

    # Federal Income and Taxes (Boxes 1-11)
    wages_tips_other_compensation = FloatField('1. Wages, tips, other compensation', validators=[DataRequired(), NumberRange(min=0)])
    federal_income_tax_withheld = FloatField('2. Federal income tax withheld', validators=[DataRequired(), NumberRange(min=0)])
    social_security_wages = FloatField('3. Social security wages', validators=[DataRequired(), NumberRange(min=0)])
    # Box 4 in prompt was Medicare wages, but official W2 is SS tax withheld. Corrected to SS wages (box 3) and Medicare wages (box 5)
    # The prompt text for boxes 3,4,5,6 was:
    # Box 3: {{ social_security_wages }}
    # Box 4: {{ medicare_wages_and_tips }}  <-- This is Box 5 on W-2
    # Box 5: {{ social_security_tax_withheld }} <-- This is Box 4 on W-2
    # Box 6: {{ medicare_tax_withheld }}
    # Adjusting form fields to match official W-2 box numbers and typical descriptions.
    medicare_wages_and_tips = FloatField('5. Medicare wages and tips', validators=[DataRequired(), NumberRange(min=0)]) # Corrected Box number based on W2 form
    social_security_tax_withheld = FloatField('4. Social security tax withheld', validators=[DataRequired(), NumberRange(min=0)]) # Corrected Box number
    medicare_tax_withheld = FloatField('6. Medicare tax withheld', validators=[DataRequired(), NumberRange(min=0)])

    social_security_tips = FloatField('7. Social security tips', validators=[Optional(), NumberRange(min=0)])
    allocated_tips = FloatField('8. Allocated tips', validators=[Optional(), NumberRange(min=0)])
    # Box 9 (Verification Code) is often not manually entered or is being phased out for general use, skip for now.
    dependent_care_benefits = FloatField('10. Dependent care benefits', validators=[Optional(), NumberRange(min=0)])
    nonqualified_plans = FloatField('11. Nonqualified plans', validators=[Optional(), NumberRange(min=0)])

    # Box 12 (Codes and Amounts) - Simplified for now with 2 fixed items.
    box_12a_code = StringField('12a. Code', validators=[Optional(), Length(max=2)]) # Max 2 chars for W2 codes like D, DD, etc.
    box_12a_amount = FloatField('12a. Amount', validators=[Optional(), NumberRange(min=0)])
    box_12b_code = StringField('12b. Code', validators=[Optional(), Length(max=2)])
    box_12b_amount = FloatField('12b. Amount', validators=[Optional(), NumberRange(min=0)])
    # Can add box_12c_code, box_12c_amount, box_12d_code, box_12d_amount if needed

    # Box 13 (Checkboxes)
    statutory_employee = BooleanField('13. Statutory employee')
    retirement_plan = BooleanField('13. Retirement plan')
    third_party_sick_pay = BooleanField('13. Third-party sick pay')

    # Box 14 (Other) - Simplified to one entry as per prompt, can be expanded or made into FieldList
    other_description_code_d = StringField('14. Other Description', validators=[Optional(), Length(max=100)])
    other_amount_code_d = FloatField('14. Other Amount', validators=[Optional(), NumberRange(min=0)])

    # State Taxes (Box 15-17) - Simplified for one state
    state_employer_state_id_no = StringField("15. Employer's state ID number", validators=[Optional(), Length(max=20)]) # Renamed from state_employer_state_id_no to match W2 box label
    state_wages_tips_etc = FloatField('16. State wages, tips, etc.', validators=[Optional(), NumberRange(min=0)])
    state_income_tax = FloatField('17. State income tax', validators=[Optional(), NumberRange(min=0)])
    # Could add fields for a second state if needed (e.g. state_employer_state_id_no_2, etc.)

    # Local Taxes (Box 18-20) - Simplified for one locality
    local_wages_tips_etc = FloatField('18. Local wages, tips, etc.', validators=[Optional(), NumberRange(min=0)])
    local_income_tax = FloatField('19. Local income tax', validators=[Optional(), NumberRange(min=0)])
    locality_name = StringField('20. Locality name', validators=[Optional(), Length(max=50)])
    # Could add fields for a second locality

    submit = SubmitField('Generate W-2 PDF')
