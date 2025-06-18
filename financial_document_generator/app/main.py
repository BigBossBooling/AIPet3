import io
import logging # Ensure logging is imported
import os # Ensure os is imported
from financial_document_generator.config import config_by_name, get_config_name # Import from config
from flask import Flask, render_template, request, send_file, abort, jsonify
from datetime import datetime
from financial_document_generator.app.generators.w2_generator import W2Generator
from financial_document_generator.app.generators.check_generator import CheckGenerator # Updated import
from financial_document_generator.app.generators.check_stub_generator import CheckStubGenerator # Updated import

# Correctly set template and static folder paths relative to the 'app' directory
# The 'app' directory is where main.py resides.
# Templates are in 'financial_document_generator/templates/'
from financial_document_generator.app.generators.bank_statement_generator import BankStatementGenerator # Added import

from financial_document_generator.app.generators.income_statement_generator import IncomeStatementGenerator # Added import

from financial_document_generator.app.generators.earning_statement_generator import EarningStatementGenerator # Added import

from .forms_wtf import W2FormWTF # Import W2FormWTF

# Static files are in 'financial_document_generator/static/'
config_name = get_config_name() # Get current config name ('development', 'testing', 'production', or 'default')
app = Flask(__name__, template_folder='../templates', static_folder='../static')
app.config.from_object(config_by_name[config_name])

# --- Helper functions for form data processing ---
def _get_form_text(form_data, key, default=''):
    value = form_data.get(key, default)
    if isinstance(value, str):
        value = value.strip()
    return value if value else default

def _get_form_float(form_data, key, default=0.0):
    val_str = form_data.get(key)
    if val_str is None or val_str.strip() == "":
        return default
    try:
        return float(val_str)
    except ValueError:
        app.logger.warning(f"Could not convert form field '{key}' value '{val_str}' to float. Using default {default}.")
        return default

def _get_form_date(form_data, key):
    date_str = form_data.get(key)
    if date_str:
        try:
            return datetime.strptime(date_str, '%Y-%m-%d').date()
        except ValueError:
            app.logger.warning(f"Could not convert form field '{key}' value '{date_str}' to date. Using None.")
            return None
    return None

# --- Logging Configuration ---
# Configure logging based on app.config
log_level = logging.DEBUG if app.config.get('DEBUG') else logging.INFO
if app.config.get('TESTING'): # Or a specific LOG_LEVEL config var
    log_level = logging.DEBUG # Often tests also want DEBUG logging

app.logger.setLevel(log_level)

# Example: Add a StreamHandler to ensure logs go to console if not already configured
# This is often handled by Flask's default logger, but can be made explicit:
# if not app.logger.handlers:
#     stream_handler = logging.StreamHandler()
#     formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
#     stream_handler.setFormatter(formatter)
#     app.logger.addHandler(stream_handler)

app.logger.info(f"Application starting with '{config_name}' configuration.")
app.logger.info(f"DEBUG mode: {app.config.get('DEBUG')}")
app.logger.info(f"TESTING mode: {app.config.get('TESTING')}")

# --- Error Handlers ---
@app.errorhandler(404)
def page_not_found(e):
    app.logger.warning(f"404 Error - Page not found: {request.url} (Referrer: {request.referrer})")
    # Simple HTML string as per prompt example, avoiding render_template for now
    return "<h1>404 - Page Not Found</h1><p>The page you are looking for does not exist.</p><p><a href='/'>Go to Home</a></p>", 404

@app.errorhandler(500)
def internal_server_error(e):
    # The original exception (e) might be a generic werkzeug exception for 500.
    # The actual exception info is in sys.exc_info() and Flask's app.logger.error already captures it with exc_info=True.
    app.logger.error(f"500 Internal Server Error triggered: {e}", exc_info=True)
    # Simple HTML string as per prompt example
    return "<h1>500 - Internal Server Error</h1><p>Sorry, something went wrong on our end. Please try again later.</p><p><a href='/'>Go to Home</a></p>", 500


@app.route('/')
def index():
    return "Financial Document Generator is running!"

@app.route('/w2', methods=['GET'])
def w2_form_page():
    form = W2FormWTF()
    return render_template('forms/w2_form.html', form=form, title="W-2 Form")

@app.route('/w2/generate', methods=['POST'])
def generate_w2_pdf_route():
    form = W2FormWTF() # This automatically gets data from request.form on POST
    if form.validate_on_submit():
        app.logger.info(f"Processing W2 form (WTF) data for employee: {form.employee_name.data}")
        data_dict = {
            'employer_ein': form.employer_ein.data,
            'employer_name': form.employer_name.data,
            'employer_address': form.employer_address.data,
            'employer_city_state_zip': form.employer_city_state_zip.data,
            'control_number': form.control_number.data,
            'employee_ssn': form.employee_ssn.data,
            'employee_name': form.employee_name.data,
            'employee_address': form.employee_address.data,
            'employee_city_state_zip': form.employee_city_state_zip.data,
            'wages_tips_other_compensation': form.wages_tips_other_compensation.data,
            'federal_income_tax_withheld': form.federal_income_tax_withheld.data,
            'social_security_wages': form.social_security_wages.data,
            'medicare_wages_and_tips': form.medicare_wages_and_tips.data,
            'social_security_tax_withheld': form.social_security_tax_withheld.data,
            'medicare_tax_withheld': form.medicare_tax_withheld.data,
            'social_security_tips': form.social_security_tips.data,
            'allocated_tips': form.allocated_tips.data,
            'dependent_care_benefits': form.dependent_care_benefits.data,
            'nonqualified_plans': form.nonqualified_plans.data,
            'box_12a_code': form.box_12a_code.data,
            'box_12a_amount': form.box_12a_amount.data,
            'box_12b_code': form.box_12b_code.data,
            'box_12b_amount': form.box_12b_amount.data,
            # Add other box 12 fields if they were added to W2FormWTF:
            # 'box_12c_code': form.box_12c_code.data,
            # 'box_12c_amount': form.box_12c_amount.data,
            # 'box_12d_code': form.box_12d_code.data,
            # 'box_12d_amount': form.box_12d_amount.data,
            'statutory_employee': form.statutory_employee.data,
            'retirement_plan': form.retirement_plan.data,
            'third_party_sick_pay': form.third_party_sick_pay.data,
            'other_description_code_d': form.other_description_code_d.data,
            'other_amount_code_d': form.other_amount_code_d.data,
            'state_employer_state_id_no': form.state_employer_state_id_no.data,
            'state_wages_tips_etc': form.state_wages_tips_etc.data,
            'state_income_tax': form.state_income_tax.data,
            'local_wages_tips_etc': form.local_wages_tips_etc.data,
            'local_income_tax': form.local_income_tax.data,
            'locality_name': form.locality_name.data,
        }
        # W2Generator's _prepare_template_data will structure this flat dict
        # e.g. for state_tax_items, box_12_items etc.

        w2_generator = W2Generator(data_dict)
        try:
            pdf_bytes = w2_generator.generate_pdf() # Get bytes
            if pdf_bytes:
                app.logger.info(f"W2 PDF generated successfully via WTForm for employee: {form.employee_name.data}")
                return send_file(
                    io.BytesIO(pdf_bytes),
                    mimetype='application/pdf',
                    as_attachment=True,
                    download_name='w2_form.pdf'
                )
            else:
                app.logger.error(f"W2 PDF generation failed (no bytes returned) for employee: {form.employee_name.data}")
                abort(500) # Or render error template
        except Exception as e:
            app.logger.error(f"Exception during W2 PDF generation (WTForm) for {form.employee_name.data}: {e}", exc_info=True)
            abort(500)
    else:
        app.logger.warning(f"W2 form (WTF) validation failed: {form.errors}")
        # Re-render the form, passing the form object to display errors
        return render_template('forms/w2_form.html', form=form, title="W-2 Form"), 400

# --- Fallback data extraction for W2 (if not using WTForms or for different content type) ---
# This is the old generate_w2_pdf_route logic, which might be removed or adapted
# if WTForms is the sole method for this route. For now, keeping it distinct means
# the old POST handling logic is still there if needed or if WTForms path is conditional.
# However, the prompt implies replacing it. I will comment it out.
#
# @app.route('/w2/generate_old', methods=['POST']) # Renamed to avoid conflict
# def generate_w2_pdf_route_old():
#     form_data = request.form
#     try:
#         app.logger.info(f"Processing W2 form data (OLD) for employee: {form_data.get('employee_name', 'N/A')}")
#         data = { ... } # Old data extraction
#     except ValueError as e:
#         app.logger.error(f"ValueError processing W2 form data (OLD): {e}", exc_info=True)
#         return f"Error in form data (OLD): one of the numeric fields has an invalid value. Details: {e}", 400
#     except Exception as e:
#         app.logger.error(f"Unexpected error processing W2 form data (OLD): {e}", exc_info=True)
#         abort(500)
#
#     w2_instance = W2Generator(data)
#     pdf_buffer = io.BytesIO()
#     try:
#         result = w2_instance.generate_pdf(output_path_or_buffer=pdf_buffer)
#         if result is False :
#              app.logger.error(f"W2 PDF generation failed (OLD) for employee {data.get('employee_name')}. Generator returned False.")
#              return "Error generating PDF.", 500
#         if pdf_buffer.getbuffer().nbytes == 0:
#             app.logger.error(f"W2 PDF generation resulted in an empty buffer (OLD) for employee {data.get('employee_name')}.")
#             return "Error generating PDF: No data written to buffer.", 500
#         pdf_buffer.seek(0)
#         app.logger.info(f"Successfully generated W2 PDF (OLD) for employee: {data.get('employee_name')}")
#     except Exception as e:
#         app.logger.error(f"Exception during W2 PDF generation (OLD) for {data.get('employee_name', 'N/A')}: {e}", exc_info=True)
#         abort(500)
#
#     return send_file(
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
        app.logger.info(f"Processing Check form data for payee: {form_data.get('payee_name', 'N/A')}")
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
        app.logger.error(f"ValueError processing Check form data: {e}", exc_info=True)
        return f"Error in form data: {e}", 400
    except Exception as e:
        app.logger.error(f"Unexpected error processing Check form data: {e}", exc_info=True)
        abort(500)

    try:
        app.logger.info(f"Generating Check PDF for payee: {data_dict.get('payee_name')}")
        generator = CheckGenerator(data=data_dict)
        pdf_bytes = generator.generate_pdf() # Call with no args to get bytes

        if not pdf_bytes or not isinstance(pdf_bytes, bytes):
            app.logger.error(f"CheckGenerator.generate_pdf did not return valid PDF bytes for payee {data_dict.get('payee_name')}.")
            return "Error generating check PDF content.", 500

        pdf_buffer = io.BytesIO(pdf_bytes)
        app.logger.info(f"Successfully generated Check PDF for payee: {data_dict.get('payee_name')}")

    except Exception as e:
        app.logger.error(f"Exception during Check PDF generation for payee {data_dict.get('payee_name', 'N/A')}: {e}", exc_info=True)
        abort(500)

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
    # Removed local get_float and get_date, using module-level _get_form_float and _get_form_date

    try:
        app.logger.info(f"Processing Check Stub form data for employee: {form_data.get('employee_name', 'N/A')}")
        data_dict = {
            'company_name': form_data.get('company_name', 'N/A'),
            'company_address': form_data.get('company_address', 'N/A'),
            'employee_name': form_data.get('employee_name', 'N/A'),
            'employee_id': form_data.get('employee_id', 'N/A'),
            'pay_period_start': _get_form_date(form_data, 'pay_period_start'),
            'pay_period_end': _get_form_date(form_data, 'pay_period_end'),
            'pay_date': _get_form_date(form_data, 'pay_date'),

            'earnings_total': _get_form_float(form_data, 'earnings_total'),
            'deductions_total': _get_form_float(form_data, 'deductions_total'),
            'net_pay': _get_form_float(form_data, 'net_pay'),

            'ytd_gross_earnings': _get_form_float(form_data, 'ytd_gross_earnings'),
            'ytd_deductions': _get_form_float(form_data, 'ytd_deductions'),
            'ytd_net_pay': _get_form_float(form_data, 'ytd_net_pay'),
        }

        # For individual earnings items (supporting the template's fallback)
        data_dict['earnings'] = {
            'regular_pay': _get_form_float(form_data, 'earnings_regular_pay'),
            'overtime_pay': _get_form_float(form_data, 'earnings_overtime_pay'),
            'bonus': _get_form_float(form_data, 'earnings_bonus'),
        }

        # Construct earnings_items list for the template (primary way if populated)
        earnings_items = []
        if data_dict['earnings']['regular_pay'] > 0 or form_data.get('earnings_regular_pay'):
            earnings_items.append({'description': 'Regular Pay', 'amount': data_dict['earnings']['regular_pay']})
        if data_dict['earnings']['overtime_pay'] > 0 or form_data.get('earnings_overtime_pay'):
            earnings_items.append({'description': 'Overtime Pay', 'amount': data_dict['earnings']['overtime_pay']})
        if data_dict['earnings']['bonus'] > 0 or form_data.get('earnings_bonus'):
            earnings_items.append({'description': 'Bonus', 'amount': data_dict['earnings']['bonus']})
        if earnings_items:
             data_dict['earnings_items'] = earnings_items


        # For individual deduction items (supporting the template's fallback)
        data_dict['deductions'] = {
            'federal_tax': _get_form_float(form_data, 'deductions_federal_tax'),
            'state_tax': _get_form_float(form_data, 'deductions_state_tax'),
            'medicare': _get_form_float(form_data, 'deductions_medicare'),
            'social_security': _get_form_float(form_data, 'deductions_social_security'),
            'other': _get_form_float(form_data, 'deductions_other'),
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
        if deduction_items:
            data_dict['deduction_items'] = deduction_items

    except Exception as e: # Catch any error during data prep
        app.logger.error(f"Error processing Check Stub form data: {e}", exc_info=True)
        # Consider if this should be a 400 or if some data prep errors are 500
        return f"Error processing form data: {e}", 400


    try:
        app.logger.info(f"Generating Check Stub PDF for employee: {data_dict.get('employee_name')}")
        generator = CheckStubGenerator(data=data_dict)
        pdf_bytes = generator.generate_pdf() # Call with no args to get bytes

        if not pdf_bytes or not isinstance(pdf_bytes, bytes):
            app.logger.error(f"CheckStubGenerator.generate_pdf did not return valid PDF bytes for employee {data_dict.get('employee_name')}.")
            return "Error generating check stub PDF content.", 500

        pdf_buffer = io.BytesIO(pdf_bytes)
        app.logger.info(f"Successfully generated Check Stub PDF for employee: {data_dict.get('employee_name')}")

    except Exception as e:
        app.logger.error(f"Exception during Check Stub PDF generation for employee {data_dict.get('employee_name', 'N/A')}: {e}", exc_info=True)
        abort(500)

    return send_file(
        pdf_buffer,
        mimetype='application/pdf',
        as_attachment=True,
        download_name='check_stub.pdf'
    )

# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
# Bank Statement Generation Routes
# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

@app.route('/bank-statement', methods=['GET'])
def show_bank_statement_form():
    """Displays the form to enter bank statement details."""
    return render_template('forms/bank_statement_form.html')

@app.route('/bank-statement/generate', methods=['POST'])
def generate_bank_statement_pdf():
    """Generates a bank statement PDF from form data."""
    form_data = request.form
    app.logger.info(f"Processing Bank Statement form data for account: {form_data.get('account_number', 'N/A')}")

    try:
        data_dict = {
            'bank_details': {
                'name': form_data.get('bank_name', 'N/A'),
                'address': form_data.get('bank_address', 'N/A'),
                'phone': form_data.get('bank_phone', 'N/A'),
                'website': form_data.get('bank_website', 'N/A'),
            },
            'account_holder': {
                'name': form_data.get('holder_name', 'N/A'),
                'address_line1': form_data.get('holder_address_line1', 'N/A'),
                'address_line2': form_data.get('holder_address_line2', 'N/A'),
            },
            'account_details': {
                'number': form_data.get('account_number', 'N/A'),
                'type': form_data.get('account_type', 'N/A'),
            },
            'statement_period': {
                'start_date': _get_form_date(form_data, 'statement_period_start_date'),
                'end_date': _get_form_date(form_data, 'statement_period_end_date'),
            },
            'statement_date': _get_form_date(form_data, 'statement_date'),
            'summary': {
                'beginning_balance': form_data.get('summary_beginning_balance', '0.00'), # Often strings in templates
                'total_deposits': form_data.get('summary_total_deposits', '0.00'),
                'total_withdrawals': form_data.get('summary_total_withdrawals', '0.00'),
                'ending_balance': form_data.get('summary_ending_balance', '0.00'),
            },
            'transactions': []
        }

        # Process up to 5 transactions
        for i in range(5):
            date_str = form_data.get(f'transaction_{i}_date')
            description = form_data.get(f'transaction_{i}_description')

            # Only add transaction if date and description are present
            if date_str and description and description.strip():
                transaction = {
                    'date': _get_form_date(form_data, f'transaction_{i}_date'), # Convert date string
                    'description': description,
                    'withdrawal': form_data.get(f'transaction_{i}_withdrawal', ''), # Keep as string or convert to float
                    'deposit': form_data.get(f'transaction_{i}_deposit', ''),     # Keep as string or convert to float
                    'balance': form_data.get(f'transaction_{i}_balance', '')      # Keep as string or convert to float
                }
                # Convert withdrawal and deposit to formatted strings or keep as is if template handles it
                # For simplicity, pass as string, template can format or generator can pre-format if needed.
                # Example if they must be floats:
                # 'withdrawal': _get_form_float(form_data, f'transaction_{i}_withdrawal', 0.0),
                # 'deposit': _get_form_float(form_data, f'transaction_{i}_deposit', 0.0),
                data_dict['transactions'].append(transaction)

    except Exception as e:
        app.logger.error(f"Error processing Bank Statement form data: {e}", exc_info=True)
        return f"Error processing form data: {e}", 400 # Or abort(500)

    try:
        app.logger.info(f"Generating Bank Statement PDF for account: {data_dict['account_details']['number']}")
        generator = BankStatementGenerator(data=data_dict)
        pdf_bytes = generator.generate_pdf() # Get bytes directly

        if not pdf_bytes or not isinstance(pdf_bytes, bytes):
            app.logger.error(f"BankStatementGenerator.generate_pdf did not return valid PDF bytes for account {data_dict['account_details']['number']}.")
            return "Error generating bank statement PDF content.", 500

        pdf_buffer = io.BytesIO(pdf_bytes)
        app.logger.info(f"Successfully generated Bank Statement PDF for account: {data_dict['account_details']['number']}")

    except Exception as e:
        app.logger.error(f"Exception during Bank Statement PDF generation for account {data_dict['account_details']['number']}: {e}", exc_info=True)
        abort(500)

    return send_file(
        pdf_buffer,
        mimetype='application/pdf',
        as_attachment=True,
        download_name='bank_statement.pdf'
    )

# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
# Income Statement Generation Routes
# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

@app.route('/income-statement', methods=['GET'])
def show_income_statement_form():
    """Displays the form to enter income statement details."""
    return render_template('forms/income_statement_form.html')

@app.route('/income-statement/generate', methods=['POST'])
def generate_income_statement_pdf():
    """Generates an income statement PDF from form data."""
    form_data = request.form
    app.logger.info(f"Processing Income Statement form data for company: {_get_form_text(form_data, 'company_name', 'N/A')}")

    try:
        data_dict = {
            'company_name': _get_form_text(form_data, 'company_name', 'N/A Company'),
            'report_title': _get_form_text(form_data, 'report_title', 'Income Statement'),
            'period_description': _get_form_text(form_data, 'period_description', 'For the Period Ended N/A'),

            'revenues_total': _get_form_float(form_data, 'revenues_total'),
            'cogs_total': _get_form_float(form_data, 'cogs_total'),

            # operating_expenses_total is optional in form; if provided, it's used.
            # Otherwise, IncomeStatementGenerator calculates it from items.
            'operating_expenses_total': _get_form_float(form_data, 'operating_expenses_total', default=None), # Pass None if not provided

            'other_income_total': _get_form_float(form_data, 'other_income_total'),
            'income_tax_expense': _get_form_float(form_data, 'income_tax_expense'),

            'operating_expense_items': []
        }
        # If operating_expenses_total was empty string, _get_form_float turns to 0.0.
        # We need to distinguish "not provided" (None) from "provided as 0".
        if 'operating_expenses_total' in form_data and form_data['operating_expenses_total'].strip() == '':
            data_dict['operating_expenses_total'] = None


        # Process operating expense items (up to 3 from the form)
        for i in range(1, 4): # Corresponds to operating_expense_1, _2, _3
            description = _get_form_text(form_data, f'operating_expense_{i}_description')
            amount_str = form_data.get(f'operating_expense_{i}_amount') # Get as string first

            if description: # Only add if description is present
                amount = 0.0
                if amount_str and amount_str.strip() != '':
                    try:
                        amount = float(amount_str)
                    except ValueError:
                        app.logger.warning(f"Could not convert op. expense amount '{amount_str}' for '{description}' to float. Using 0.0.")

                data_dict['operating_expense_items'].append({
                    'description': description,
                    'amount': amount
                })

    except Exception as e:
        app.logger.error(f"Error processing Income Statement form data: {e}", exc_info=True)
        return f"Error processing form data: {e}", 400

    try:
        company_name_log = data_dict.get('company_name', 'N/A')
        app.logger.info(f"Generating Income Statement PDF for company: {company_name_log}")
        generator = IncomeStatementGenerator(data=data_dict) # Generator will calculate derived totals
        pdf_bytes = generator.generate_pdf()

        if not pdf_bytes or not isinstance(pdf_bytes, bytes):
            app.logger.error(f"IncomeStatementGenerator.generate_pdf did not return valid PDF bytes for {company_name_log}.")
            return "Error generating income statement PDF content.", 500

        pdf_buffer = io.BytesIO(pdf_bytes)
        app.logger.info(f"Successfully generated Income Statement PDF for company: {company_name_log}")

    except Exception as e:
        app.logger.error(f"Exception during Income Statement PDF generation for {data_dict.get('company_name', 'N/A')}: {e}", exc_info=True)
        abort(500)

    return send_file(
        pdf_buffer,
        mimetype='application/pdf',
        as_attachment=True,
        download_name='income_statement.pdf'
    )

# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
# Earning Statement Generation Routes
# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

@app.route('/earning-statement', methods=['GET'])
def show_earning_statement_form():
    """Displays the form to enter earning statement details."""
    return render_template('forms/earning_statement_form.html')

@app.route('/earning-statement/generate', methods=['POST'])
def generate_earning_statement_pdf():
    """Generates an earning statement PDF from form data."""
    form_data = request.form
    employee_name_log = _get_form_text(form_data, 'employee_name', 'N/A')
    app.logger.info(f"Processing Earning Statement form data for employee: {employee_name_log}")

    try:
        data_dict = {
            'company_name': _get_form_text(form_data, 'company_name', 'N/A Company'),
            'company_address': _get_form_text(form_data, 'company_address', 'N/A Address'),
            'employee_name': employee_name_log,
            'employee_id': _get_form_text(form_data, 'employee_id', 'N/A'),
            'pay_period_start': _get_form_date(form_data, 'pay_period_start'),
            'pay_period_end': _get_form_date(form_data, 'pay_period_end'),
            'pay_date': _get_form_date(form_data, 'pay_date'),

            'earnings_total': _get_form_float(form_data, 'earnings_total'),
            'deductions_total': _get_form_float(form_data, 'deductions_total'),
            'net_pay': _get_form_float(form_data, 'net_pay'),

            'ytd_gross_earnings': _get_form_float(form_data, 'ytd_gross_earnings'),
            'ytd_deductions': _get_form_float(form_data, 'ytd_deductions'),
            'ytd_net_pay': _get_form_float(form_data, 'ytd_net_pay'),

            'corporate_notes': _get_form_text(form_data, 'corporate_notes', '')
        }

        # Process earnings items (similar to check stub)
        data_dict['earnings'] = {
            'regular_pay': _get_form_float(form_data, 'earnings_regular_pay'),
            'overtime_pay': _get_form_float(form_data, 'earnings_overtime_pay'),
            'bonus': _get_form_float(form_data, 'earnings_bonus'),
        }
        earnings_items = []
        if data_dict['earnings']['regular_pay'] > 0 or form_data.get('earnings_regular_pay'):
            earnings_items.append({'description': 'Regular Pay', 'amount': data_dict['earnings']['regular_pay']})
        if data_dict['earnings']['overtime_pay'] > 0 or form_data.get('earnings_overtime_pay'):
            earnings_items.append({'description': 'Overtime Pay', 'amount': data_dict['earnings']['overtime_pay']})
        if data_dict['earnings']['bonus'] > 0 or form_data.get('earnings_bonus'):
            earnings_items.append({'description': 'Bonus', 'amount': data_dict['earnings']['bonus']})
        if earnings_items:
            data_dict['earnings_items'] = earnings_items

        # Process deduction items (similar to check stub)
        data_dict['deductions'] = {
            'federal_tax': _get_form_float(form_data, 'deductions_federal_tax'),
            'state_tax': _get_form_float(form_data, 'deductions_state_tax'),
            'medicare': _get_form_float(form_data, 'deductions_medicare'),
            'social_security': _get_form_float(form_data, 'deductions_social_security'),
            'other': _get_form_float(form_data, 'deductions_other'),
        }
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
        if deduction_items:
            data_dict['deduction_items'] = deduction_items

    except Exception as e:
        app.logger.error(f"Error processing Earning Statement form data for employee {employee_name_log}: {e}", exc_info=True)
        return f"Error processing form data: {e}", 400

    try:
        app.logger.info(f"Generating Earning Statement PDF for employee: {employee_name_log}")
        generator = EarningStatementGenerator(data=data_dict)
        pdf_bytes = generator.generate_pdf()

        if not pdf_bytes or not isinstance(pdf_bytes, bytes):
            app.logger.error(f"EarningStatementGenerator.generate_pdf did not return valid PDF bytes for {employee_name_log}.")
            return "Error generating earning statement PDF content.", 500

        pdf_buffer = io.BytesIO(pdf_bytes)
        app.logger.info(f"Successfully generated Earning Statement PDF for employee: {employee_name_log}")

    except Exception as e:
        app.logger.error(f"Exception during Earning Statement PDF generation for {employee_name_log}: {e}", exc_info=True)
        abort(500)

    return send_file(
        pdf_buffer,
        mimetype='application/pdf',
        as_attachment=True,
        download_name='earning_statement.pdf'
    )

# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
# API Routes
# +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

@app.route('/api/v1/w2/generate', methods=['POST'])
def api_generate_w2_pdf():
    """
    API endpoint to generate a W2 PDF from JSON data.
    """
    app.logger.info("Received API request for W2 PDF generation.")

    if not request.is_json:
        app.logger.warning("API request for W2 generation is not JSON.")
        return jsonify({'error': 'Request must be JSON'}), 415

    json_data = request.get_json()
    if json_data is None:
        app.logger.warning("API request for W2 generation contained invalid JSON payload.")
        return jsonify({'error': 'Invalid JSON payload'}), 400

    # Basic Validation & Data Extraction
    required_str_fields = ['employee_name', 'employee_ssn', 'employer_name', 'employer_ein']
    required_num_fields = ['wages_tips_other_compensation', 'federal_income_tax_withheld']
    optional_str_fields = ['state_employer_state_id_no', 'locality_name']
    optional_num_fields = [
        'social_security_wages', 'medicare_wages_and_tips',
        'social_security_tax_withheld', 'medicare_tax_withheld',
        'state_wages_tips_etc', 'state_income_tax',
        'local_wages_tips_etc', 'local_income_tax'
    ]

    data_dict = {}
    try:
        for field in required_str_fields:
            value = json_data.get(field)
            if not value or not isinstance(value, str) or not value.strip():
                app.logger.error(f"API W2 generation: Missing or invalid required string field: {field}")
                return jsonify({'error': f'Missing or invalid required field: {field}'}), 400
            data_dict[field] = value.strip()

        for field in required_num_fields:
            value = json_data.get(field)
            if value is None: # Allow 0, but not missing
                app.logger.error(f"API W2 generation: Missing required numeric field: {field}")
                return jsonify({'error': f'Missing required field: {field}'}), 400
            try:
                data_dict[field] = float(value)
            except (ValueError, TypeError):
                app.logger.error(f"API W2 generation: Invalid type for numeric field: {field} (value: {value})")
                return jsonify({'error': f'Invalid type for numeric field: {field}'}), 400

        for field in optional_str_fields:
            data_dict[field] = str(json_data.get(field, '')).strip()

        for field in optional_num_fields:
            value = json_data.get(field, 0.0)
            try:
                data_dict[field] = float(value)
            except (ValueError, TypeError):
                app.logger.warning(f"API W2 generation: Invalid type for optional numeric field: {field} (value: {value}). Defaulting to 0.0.")
                data_dict[field] = 0.0

    except Exception as e: # Catch any other unexpected error during data extraction
        app.logger.error(f"Unexpected error during API W2 data extraction: {e}", exc_info=True)
        return jsonify({'error': 'Error processing request data.'}), 400

    app.logger.info(f"API W2 generation: Data validated for employee: {data_dict.get('employee_name')}")

    try:
        w2_generator = W2Generator(data_dict)
        pdf_bytes = w2_generator.generate_pdf() # Get bytes

        if not pdf_bytes:
            app.logger.error(f"API W2 generation: W2Generator.generate_pdf returned None for employee {data_dict.get('employee_name')}")
            return jsonify({'error': 'Failed to generate W2 PDF'}), 500

        app.logger.info(f"API W2 generation: Successfully generated PDF for employee: {data_dict.get('employee_name')}")
        return send_file(
            io.BytesIO(pdf_bytes),
            mimetype='application/pdf',
            as_attachment=True,
            download_name='w2_form.pdf'
        )
    except Exception as e:
        app.logger.error(f"API W2 generation: Exception during PDF generation or sending for employee {data_dict.get('employee_name')}: {e}", exc_info=True)
        # Do not return jsonify here if already sent headers or if it's an unhandled exception type for Flask.
        # The @app.errorhandler(500) should ideally catch this if it's an unhandled server error.
        # However, if send_file fails mid-stream, it's complex. For now, let 500 handler try.
        abort(500)

@app.route('/api/v1/earning-statement/generate', methods=['POST'])
def api_generate_earning_statement_pdf():
    """
    API endpoint to generate an Earning Statement PDF from JSON data.
    """
    app.logger.info("Received API request for Earning Statement PDF generation.")

    if not request.is_json:
        app.logger.warning("API request for Earning Statement generation is not JSON.")
        return jsonify({'error': 'Request must be JSON'}), 415

    json_data = request.get_json()
    if json_data is None:
        app.logger.warning("API request for Earning Statement generation contained invalid JSON payload.")
        return jsonify({'error': 'Invalid JSON payload'}), 400

    # Define required and optional fields based on EarningStatementGenerator and its template
    # This is similar to CheckStub data structure
    required_str_fields = ['company_name', 'employee_name', 'pay_period_start', 'pay_period_end', 'pay_date']
    required_num_fields = ['earnings_total', 'deductions_total', 'net_pay'] # Assuming these totals are required

    optional_str_fields = ['company_address', 'employee_id', 'corporate_notes']
    optional_num_fields = [ # Individual earnings/deductions can be optional if totals are given, or vice-versa
        'earnings.regular_pay', 'earnings.overtime_pay', 'earnings.bonus',
        'deductions.federal_tax', 'deductions.state_tax', 'deductions.medicare',
        'deductions.social_security', 'deductions.other',
        'ytd_gross_earnings', 'ytd_deductions', 'ytd_net_pay'
    ]
    # Itemized lists are also optional and can be processed if provided
    # 'earnings_items': [{'description': 'desc', 'amount': 0.0}, ...],
    # 'deduction_items': [{'description': 'desc', 'amount': 0.0}, ...],

    data_dict = {}
    try:
        for field in required_str_fields:
            value = json_data.get(field)
            if field in ['pay_period_start', 'pay_period_end', 'pay_date']: # These are date strings
                if not value or not isinstance(value, str) or not value.strip():
                    app.logger.error(f"API Earning Stmt: Missing or invalid required date string field: {field}")
                    return jsonify({'error': f'Missing or invalid required field: {field} (expected YYYY-MM-DD string)'}), 400
                try:
                    data_dict[field] = datetime.strptime(value.strip(), '%Y-%m-%d').date()
                except ValueError:
                    app.logger.error(f"API Earning Stmt: Invalid date format for {field}: {value}")
                    return jsonify({'error': f'Invalid date format for {field}. Expected YYYY-MM-DD.'}), 400
            else: # Other string fields
                if not value or not isinstance(value, str) or not value.strip():
                    app.logger.error(f"API Earning Stmt: Missing or invalid required string field: {field}")
                    return jsonify({'error': f'Missing or invalid required field: {field}'}), 400
                data_dict[field] = value.strip()

        for field in required_num_fields:
            value = json_data.get(field)
            if value is None:
                app.logger.error(f"API Earning Stmt: Missing required numeric field: {field}")
                return jsonify({'error': f'Missing required field: {field}'}), 400
            try:
                data_dict[field] = float(value)
            except (ValueError, TypeError):
                app.logger.error(f"API Earning Stmt: Invalid type for numeric field: {field} (value: {value})")
                return jsonify({'error': f'Invalid type for numeric field: {field}'}), 400

        for field in optional_str_fields:
            data_dict[field] = str(json_data.get(field, '')).strip()

        # Handling nested optional numeric fields (e.g., earnings.regular_pay)
        # and top-level optional numeric fields (e.g., ytd_gross_earnings)
        data_dict['earnings'] = {}
        data_dict['deductions'] = {}

        nested_optional_num_fields_map = {
            'earnings.regular_pay': ('earnings','regular_pay'), 'earnings.overtime_pay': ('earnings','overtime_pay'),
            'earnings.bonus': ('earnings','bonus'), 'deductions.federal_tax': ('deductions','federal_tax'),
            'deductions.state_tax': ('deductions','state_tax'), 'deductions.medicare': ('deductions','medicare'),
            'deductions.social_security': ('deductions','social_security'), 'deductions.other': ('deductions','other')
        }

        for json_key, (outer_key, inner_key) in nested_optional_num_fields_map.items():
            value = json_data.get(json_key, 0.0) # Default to 0.0 if not present
            try:
                data_dict[outer_key][inner_key] = float(value)
            except (ValueError, TypeError):
                app.logger.warning(f"API Earning Stmt: Invalid type for optional field {json_key} (value: {value}). Defaulting to 0.0.")
                data_dict[outer_key][inner_key] = 0.0

        ytd_fields = ['ytd_gross_earnings', 'ytd_deductions', 'ytd_net_pay']
        for field in ytd_fields:
            value = json_data.get(field, 0.0)
            try:
                data_dict[field] = float(value)
            except (ValueError, TypeError):
                app.logger.warning(f"API Earning Stmt: Invalid type for YTD field {field} (value: {value}). Defaulting to 0.0.")
                data_dict[field] = 0.0

        # Process itemized lists if provided
        for item_list_name in ['earnings_items', 'deduction_items']:
            items_data = json_data.get(item_list_name)
            if isinstance(items_data, list):
                processed_items = []
                for item in items_data:
                    if isinstance(item, dict) and 'description' in item and 'amount' in item:
                        try:
                            amount = float(item['amount'])
                            processed_items.append({'description': str(item['description']), 'amount': amount})
                        except (ValueError, TypeError):
                            app.logger.warning(f"API Earning Stmt: Invalid amount in {item_list_name} item: {item}. Skipping.")
                    else:
                        app.logger.warning(f"API Earning Stmt: Invalid item structure in {item_list_name}: {item}. Skipping.")
                if processed_items: # Only add if there are valid items
                    data_dict[item_list_name] = processed_items
            elif items_data is not None: # If key exists but is not a list
                 app.logger.warning(f"API Earning Stmt: {item_list_name} should be a list, but got {type(items_data)}. Ignoring.")


    except Exception as e:
        app.logger.error(f"Unexpected error during API Earning Statement data extraction: {e}", exc_info=True)
        return jsonify({'error': 'Error processing request data.'}), 400

    employee_name_log = data_dict.get('employee_name', 'N/A')
    app.logger.info(f"API Earning Statement generation: Data validated for employee: {employee_name_log}")

    try:
        generator = EarningStatementGenerator(data_dict)
        pdf_bytes = generator.generate_pdf()

        if not pdf_bytes:
            app.logger.error(f"API Earning Stmt: EarningStatementGenerator.generate_pdf returned None for {employee_name_log}")
            return jsonify({'error': 'Failed to generate Earning Statement PDF'}), 500

        app.logger.info(f"API Earning Stmt: Successfully generated PDF for employee: {employee_name_log}")
        return send_file(
            io.BytesIO(pdf_bytes),
            mimetype='application/pdf',
            as_attachment=True,
            download_name='earning_statement.pdf'
        )
    except Exception as e:
        app.logger.error(f"API Earning Stmt: Exception during PDF generation or sending for {employee_name_log}: {e}", exc_info=True)
        abort(500)

@app.route('/api/v1/check/generate', methods=['POST'])
def api_generate_check_pdf():
    """
    API endpoint to generate a Check PDF from JSON data.
    """
    app.logger.info("Received API request for Check PDF generation.")

    if not request.is_json:
        app.logger.warning("API request for Check generation is not JSON.")
        return jsonify({'error': 'Request must be JSON'}), 415

    json_data = request.get_json()
    if json_data is None:
        app.logger.warning("API request for Check generation contained invalid JSON payload.")
        return jsonify({'error': 'Invalid JSON payload'}), 400

    required_str_fields = ['bank_name', 'payee_name', 'amount_words', 'routing_number', 'account_number', 'check_number']
    # amount_numeric is special: it's required, and should be a string representing a number.
    optional_str_fields = ['bank_address', 'memo', 'bank_logo_url', 'date_output_format']
    optional_date_fields = ['date'] # Expected as 'YYYY-MM-DD' string

    data_dict = {}
    try:
        for field in required_str_fields:
            value = json_data.get(field)
            if not value or not isinstance(value, str) or not value.strip():
                app.logger.error(f"API Check generation: Missing or invalid required string field: {field}")
                return jsonify({'error': f'Missing or invalid required field: {field}'}), 400
            data_dict[field] = value.strip()

        # Handle amount_numeric (required, string representation of a float)
        amount_numeric_val = json_data.get('amount_numeric')
        if amount_numeric_val is None:
            app.logger.error("API Check generation: Missing required field: amount_numeric")
            return jsonify({'error': 'Missing required field: amount_numeric'}), 400
        if isinstance(amount_numeric_val, (int, float)):
            # Format to string with 2 decimal places if it's a number
            data_dict['amount_numeric'] = f"{float(amount_numeric_val):.2f}"
        elif isinstance(amount_numeric_val, str):
            # Validate if it's a string that looks like a number
            try:
                # Ensure it can be converted to float, then store as string
                _ = float(amount_numeric_val)
                data_dict['amount_numeric'] = amount_numeric_val.strip()
            except ValueError:
                app.logger.error(f"API Check generation: Invalid format for amount_numeric: {amount_numeric_val}")
                return jsonify({'error': 'Invalid format for amount_numeric. Must be a string representing a number (e.g., "123.45").'}), 400
        else:
            app.logger.error(f"API Check generation: Invalid type for amount_numeric: {amount_numeric_val}")
            return jsonify({'error': 'Invalid type for amount_numeric. Must be string or number.'}), 400


        for field in optional_str_fields:
            data_dict[field] = str(json_data.get(field, '')).strip()
            if field == 'bank_logo_url' and not data_dict[field]: # Ensure None if empty string for logo
                data_dict[field] = None


        for field in optional_date_fields:
            date_str = json_data.get(field)
            if date_str:
                if not isinstance(date_str, str):
                    app.logger.error(f"API Check generation: Invalid type for date field {field}. Expected YYYY-MM-DD string.")
                    return jsonify({'error': f'Invalid type for date field: {field}. Expected YYYY-MM-DD string.'}), 400
                try:
                    data_dict[field] = datetime.strptime(date_str, '%Y-%m-%d').date()
                except ValueError:
                    app.logger.error(f"API Check generation: Invalid date format for {field}: {date_str}. Expected YYYY-MM-DD.")
                    return jsonify({'error': f'Invalid date format for {field}. Expected YYYY-MM-DD.'}), 400
            else:
                data_dict[field] = None # Default to None if not provided

    except Exception as e:
        app.logger.error(f"Unexpected error during API Check data extraction: {e}", exc_info=True)
        return jsonify({'error': 'Error processing request data.'}), 400

    app.logger.info(f"API Check generation: Data validated for payee: {data_dict.get('payee_name')}")

    try:
        check_generator = CheckGenerator(data_dict)
        pdf_bytes = check_generator.generate_pdf()

        if not pdf_bytes:
            app.logger.error(f"API Check generation: CheckGenerator.generate_pdf returned None for payee {data_dict.get('payee_name')}")
            return jsonify({'error': 'Failed to generate Check PDF'}), 500

        app.logger.info(f"API Check generation: Successfully generated PDF for payee: {data_dict.get('payee_name')}")
        return send_file(
            io.BytesIO(pdf_bytes),
            mimetype='application/pdf',
            as_attachment=True,
            download_name='check.pdf'
        )
    except Exception as e:
        app.logger.error(f"API Check generation: Exception during PDF generation or sending for payee {data_dict.get('payee_name')}: {e}", exc_info=True)
        abort(500)
