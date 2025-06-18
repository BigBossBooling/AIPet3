import io
from flask import Flask, render_template, request, send_file
from financial_document_generator.app.generators.w2_generator import W2Generator

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

    w2_instance = W2Generator(**data)

    pdf_buffer = io.BytesIO()
    try:
        w2_instance.generate_pdf(pdf_buffer)
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
