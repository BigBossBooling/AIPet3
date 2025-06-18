import io
import logging
from flask import Flask, render_template, request, send_file, abort, jsonify # Added jsonify
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

# Static files are in 'financial_document_generator/static/'
app = Flask(__name__, template_folder='../templates', static_folder='../static')

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
# Configure logging before routes are defined, after app instantiation
if not app.debug: # Example: More structured logging for production
    # For simplicity in this step, we'll rely on Flask's default logger which logs to stderr.
    # In a real app, you might add file handlers, formatters, etc.
    # e.g., from logging.handlers import RotatingFileHandler
    # file_handler = RotatingFileHandler('app.log', maxBytes=10240, backupCount=10)
    # file_handler.setFormatter(logging.Formatter(
    #    '%(asctime)s %(levelname)s: %(message)s [in %(pathname)s:%(lineno)d]'
    # ))
    # file_handler.setLevel(logging.INFO)
    # app.logger.addHandler(file_handler)
    app.logger.setLevel(logging.INFO)
else: # For debug mode, Flask's default logger is usually sufficient and quite verbose.
    app.logger.setLevel(logging.DEBUG)

app.logger.info('Financial Document Generator application starting up...')

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
        app.logger.info(f"Processing W2 form data for employee: {form_data.get('employee_name', 'N/A')}")
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
        app.logger.error(f"ValueError processing W2 form data: {e}", exc_info=True)
        return f"Error in form data: one of the numeric fields has an invalid value. Details: {e}", 400
    except Exception as e:
        app.logger.error(f"Unexpected error processing W2 form data: {e}", exc_info=True)
        abort(500) # Let the 500 handler take over

    w2_instance = W2Generator(data)

    pdf_buffer = io.BytesIO()
    try:
        # W2Generator.generate_pdf will write to pdf_buffer if it's passed as an argument.
        # It returns None in this case on success (for buffer write), or False on error (for path write).
        result = w2_instance.generate_pdf(output_path_or_buffer=pdf_buffer)

        # For buffer operations, a successful write often means the method doesn't return an error,
        # and the buffer itself is modified. W2Generator returns None for successful buffer write.
        # It returns False for path write errors, or None for other internal errors.
        if result is False : # Explicit False means path write error or other specific error.
             app.logger.error(f"W2 PDF generation failed for employee {data.get('employee_name')}. Generator returned False.")
             return "Error generating PDF.", 500

        if pdf_buffer.getbuffer().nbytes == 0:
            app.logger.error(f"W2 PDF generation resulted in an empty buffer for employee {data.get('employee_name')}.")
            return "Error generating PDF: No data written to buffer.", 500

        pdf_buffer.seek(0)
        app.logger.info(f"Successfully generated W2 PDF for employee: {data.get('employee_name')}")
    except Exception as e:
        app.logger.error(f"Exception during W2 PDF generation for employee {data.get('employee_name', 'N/A')}: {e}", exc_info=True)
        abort(500) # Let the 500 handler take over

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
