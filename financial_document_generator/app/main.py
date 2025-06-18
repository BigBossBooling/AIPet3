import io
from flask import Flask, render_template, request, send_file
from datetime import datetime # Added for date conversion
from financial_document_generator.app.generators.w2_generator import W2Generator
from financial_document_generator.app.generators.check_generator import CheckGenerator # Updated import
from financial_document_generator.app.generators.check_stub_generator import CheckStubGenerator # Updated import

# Correctly set template and static folder paths relative to the 'app' directory
# The 'app' directory is where main.py resides.
# Templates are in 'financial_document_generator/templates/'
# Static files are in 'financial_document_generator/static/'
app = Flask(__name__, template_folder='../templates', static_folder='../static')

@app.route('/')
def index():
    return "Financial Document Generator is running!"

@app.route('/w2', methods=['GET'])
def w2_form_page():
    # The template path is relative to the `template_folder` defined above.
    # So, 'forms/w2_form.html' will resolve to '../templates/forms/w2_form.html'
    # which correctly points to 'financial_document_generator/templates/forms/w2_form.html'
    return render_template('forms/w2_form.html')

@app.route('/w2/generate', methods=['POST'])
def generate_w2_pdf_route():
    form_data = request.form

    # Convert form data to the types expected by W2Generator
    # It's crucial to handle potential ValueError if conversion fails for numeric fields
    try:
        data = {
            "employee_name": form_data.get("employee_name", ""),
            "employee_ssn": form_data.get("employee_ssn", ""),
            "employer_name": form_data.get("employer_name", ""),
            "employer_ein": form_data.get("employer_ein", ""),
            "wages_tips_other_compensation": float(form_data.get("wages_tips_other_compensation", 0.0)),
            "federal_income_tax_withheld": float(form_data.get("federal_income_tax_withheld", 0.0)),
            "social_security_wages": float(form_data.get("social_security_wages", 0.0)),
            "medicare_wages_and_tips": float(form_data.get("medicare_wages_and_tips", 0.0)),
            "social_security_tax_withheld": float(form_data.get("social_security_tax_withheld", 0.0)),
            "medicare_tax_withheld": float(form_data.get("medicare_tax_withheld", 0.0)),
            "state_employer_state_id_no": form_data.get("state_employer_state_id_no", ""),
            "state_wages_tips_etc": float(form_data.get("state_wages_tips_etc", 0.0)),
            "state_income_tax": float(form_data.get("state_income_tax", 0.0)),
            "local_wages_tips_etc": float(form_data.get("local_wages_tips_etc", 0.0)),
            "local_income_tax": float(form_data.get("local_income_tax", 0.0)),
            "locality_name": form_data.get("locality_name", "")
        }
    except ValueError as e:
        return f"Error in form data: one of the numeric fields has an invalid value. Details: {e}", 400

    w2_instance = W2Generator(data) # Changed from **data to data

    pdf_buffer = io.BytesIO()
    try:
        # W2Generator.generate_pdf will write to pdf_buffer if it's passed as an argument.
        # It returns None in this case on success, or False/None on error.
        result = w2_instance.generate_pdf(output_path_or_buffer=pdf_buffer)
        if result is False: # Explicitly False indicates an error during PDF generation for path, check W2Generator.
             print(f"Error during PDF generation: generate_pdf returned False")
             return "Error generating PDF.", 500
        # If result is None, it means it either successfully wrote to buffer or it was an error for byte return.
        # Given we pass a buffer, None means success.
        # We need to ensure the buffer was actually written to.
        if pdf_buffer.getbuffer().nbytes == 0:
            print(f"Error during PDF generation: Buffer is empty after call.")
            return "Error generating PDF: No data written to buffer.", 500

        pdf_buffer.seek(0) # Rewind the buffer to the beginning
    except Exception as e:
        # Log the exception e for debugging
        print(f"Error during PDF generation: {e}")
        return "Error generating PDF.", 500

    return send_file(
        pdf_buffer,
        as_attachment=True,
        download_name='w2_form.pdf', # Changed from filename to download_name for newer Flask versions
        mimetype='application/pdf'
    )

if __name__ == '__main__':
    # Note: For production, use a WSGI server like Gunicorn or Waitress
    # instead of Flask's built-in development server.
    app.run(debug=True, host='0.0.0.0', port=5000)


# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
# Check Generation Routes
# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

@app.route('/check', methods=['GET'])
def show_check_form():
    """Displays the form to enter check details."""
    return render_template('forms/check_form.html')

@app.route('/check/generate', methods=['POST'])
def generate_check_pdf():
    """Generates a check PDF from form data."""
    form_data = request.form

    try:
        # Prepare data for the generate_check function
        # Date conversion: HTML date input typically submits as 'YYYY-MM-DD'
        date_str = form_data.get('date')
        check_date = datetime.strptime(date_str, '%Y-%m-%d').date() if date_str else None

        data_dict = {
            'bank_name': form_data.get('bank_name', 'N/A'),
            'bank_address': form_data.get('bank_address', 'N/A'),
            'check_number': form_data.get('check_number', 'N/A'),
            'date': check_date,
            'payee_name': form_data.get('payee_name', 'N/A'),
            'amount_numeric': form_data.get('amount_numeric', '0.00'), # Keep as string, template handles formatting
            'amount_words': form_data.get('amount_words', 'Zero and 00/100'),
            'memo': form_data.get('memo', ''),
            'routing_number': form_data.get('routing_number', 'N/A'),
            'account_number': form_data.get('account_number', 'N/A'),
            'bank_logo_url': form_data.get('bank_logo_url') if form_data.get('bank_logo_url') else None,
            # 'font_configs': [], # Optional: Add if you allow font selection in the form
            # 'date_output_format': None # Optional: Or get from form if needed
        }
    except ValueError as e:
        # Handle issues like invalid date format
        print(f"Error processing form data for check: {e}")
        return f"Error in form data: {e}", 400
    except Exception as e:
        print(f"Unexpected error processing form data for check: {e}")
        return "An unexpected error occurred while processing form data.", 500

    try:
        generator = CheckGenerator(data=data_dict)
        pdf_bytes = generator.generate_pdf() # Call with no args to get bytes

        if not pdf_bytes or not isinstance(pdf_bytes, bytes): # Check if pdf_bytes is None or not bytes
            print("Error: CheckGenerator.generate_pdf did not return valid PDF bytes.")
            return "Error generating check PDF content.", 500

        pdf_buffer = io.BytesIO(pdf_bytes)
        # No need to seek(0) here as BytesIO(bytes) is already at position 0

    except Exception as e:
        print(f"Error during check PDF generation: {e}")
        # Potentially log the full traceback here
        return "Error generating check PDF.", 500

    return send_file(
        pdf_buffer,
        mimetype='application/pdf',
        as_attachment=True,
        download_name='check.pdf'
    )

# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
# Check Stub Generation Routes
# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

@app.route('/check-stub', methods=['GET'])
def show_check_stub_form():
    """Displays the form to enter check stub details."""
    return render_template('forms/check_stub_form.html')

@app.route('/check-stub/generate', methods=['POST'])
def generate_check_stub_pdf():
    """Generates a check stub PDF from form data."""
    form_data = request.form

    def get_float(key, default=0.0):
        val_str = form_data.get(key)
        if val_str is None or val_str.strip() == "":
            return default
        try:
            return float(val_str)
        except ValueError:
            # Potentially raise an error or return a specific indicator if needed
            print(f"Warning: Could not convert form field '{key}' value '{val_str}' to float. Using default {default}.")
            return default

    def get_date(key):
        date_str = form_data.get(key)
        if date_str:
            try:
                return datetime.strptime(date_str, '%Y-%m-%d').date()
            except ValueError:
                print(f"Warning: Could not convert form field '{key}' value '{date_str}' to date. Using None.")
                return None
        return None

    try:
        data_dict = {
            'company_name': form_data.get('company_name', 'N/A'),
            'company_address': form_data.get('company_address', 'N/A'),
            'employee_name': form_data.get('employee_name', 'N/A'),
            'employee_id': form_data.get('employee_id', 'N/A'),
            'pay_period_start': get_date('pay_period_start'),
            'pay_period_end': get_date('pay_period_end'),
            'pay_date': get_date('pay_date'),

            'earnings_total': get_float('earnings_total'),
            'deductions_total': get_float('deductions_total'),
            'net_pay': get_float('net_pay'),

            'ytd_gross_earnings': get_float('ytd_gross_earnings'),
            'ytd_deductions': get_float('ytd_deductions'),
            'ytd_net_pay': get_float('ytd_net_pay'),
        }

        # For individual earnings items (supporting the template's fallback)
        data_dict['earnings'] = {
            'regular_pay': get_float('earnings_regular_pay'),
            'overtime_pay': get_float('earnings_overtime_pay'),
            'bonus': get_float('earnings_bonus'),
        }

        # Construct earnings_items list for the template (primary way if populated)
        earnings_items = []
        if data_dict['earnings']['regular_pay'] > 0 or form_data.get('earnings_regular_pay'): # check form data to include if zero explicitly entered
            earnings_items.append({'description': 'Regular Pay', 'amount': data_dict['earnings']['regular_pay']})
        if data_dict['earnings']['overtime_pay'] > 0 or form_data.get('earnings_overtime_pay'):
            earnings_items.append({'description': 'Overtime Pay', 'amount': data_dict['earnings']['overtime_pay']})
        if data_dict['earnings']['bonus'] > 0 or form_data.get('earnings_bonus'):
            earnings_items.append({'description': 'Bonus', 'amount': data_dict['earnings']['bonus']})
        if earnings_items: # Only add if there's something, otherwise template fallback for earnings might be cleaner
             data_dict['earnings_items'] = earnings_items


        # For individual deduction items (supporting the template's fallback)
        data_dict['deductions'] = {
            'federal_tax': get_float('deductions_federal_tax'),
            'state_tax': get_float('deductions_state_tax'),
            'medicare': get_float('deductions_medicare'),
            'social_security': get_float('deductions_social_security'),
            'other': get_float('deductions_other'),
        }

        # Construct deduction_items list for the template
        deduction_items = []
        if data_dict['deductions']['federal_tax'] > 0 or form_data.get('deductions_federal_tax'):
            deduction_items.append({'description': 'Federal Tax', 'amount': data_dict['deductions']['federal_tax']})
        if data_dict['deductions']['state_tax'] > 0 or form_data.get('deductions_state_tax'):
            deduction_items.append({'description': 'State Tax', 'amount': data_dict['deductions']['state_tax']})
        if data_dict['deductions']['medicare'] > 0 or form_data.get('deductions_medicare'):
            deduction_items.append({'description': 'Medicare', 'amount': data_dict['deductions']['medicare']})
        if data_dict['deductions']['social_security'] > 0 or form_data.get('deductions_social_security'):
            deduction_items.append({'description': 'Social Security', 'amount': data_dict['deductions']['social_security']})
        if data_dict['deductions']['other'] > 0 or form_data.get('deductions_other'):
            deduction_items.append({'description': 'Other Deductions', 'amount': data_dict['deductions']['other']})
        if deduction_items: # Only add if there's something
            data_dict['deduction_items'] = deduction_items

    except Exception as e: # Catch any error during data prep
        print(f"Error processing form data for check stub: {e}")
        return f"Error processing form data: {e}", 400

    try:
        generator = CheckStubGenerator(data=data_dict)
        pdf_bytes = generator.generate_pdf() # Call with no args to get bytes

        if not pdf_bytes or not isinstance(pdf_bytes, bytes): # Check if pdf_bytes is None or not bytes
            print("Error: CheckStubGenerator.generate_pdf did not return valid PDF bytes.")
            return "Error generating check stub PDF content.", 500

        pdf_buffer = io.BytesIO(pdf_bytes)
        # No need to seek(0) here as BytesIO(bytes) is already at position 0

    except Exception as e:
        print(f"Error during check stub PDF generation: {e}")
        return "Error generating check stub PDF.", 500

    return send_file(
        pdf_buffer,
        mimetype='application/pdf',
        as_attachment=True,
        download_name='check_stub.pdf'
    )
