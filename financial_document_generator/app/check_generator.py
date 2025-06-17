import os
from jinja2 import Environment, FileSystemLoader
from weasyprint import HTML, CSS
from weasyprint.text.fonts import FontConfiguration

def generate_check(data, output_path=None):
    '''
    Generates a check PDF from data.

    Args:
        data (dict): A dictionary containing check details.
                     Expected keys: bank_name, bank_address, check_number, date,
                                      payee_name, amount_numeric, amount_words, memo,
                                      routing_number, account_number.
        output_path (str, optional): If provided, saves the PDF to this path.
                                     Otherwise, returns the PDF as bytes.

    Returns:
        bytes or None: PDF content as bytes if output_path is None, else None if file is saved.
                       Returns None on error as well.
    '''
    try:
        # Correctly determine the base directory of the project
        # __file__ is financial_document_generator/app/check_generator.py
        # os.path.dirname(__file__) is financial_document_generator/app/
        # os.path.dirname(os.path.dirname(__file__)) is financial_document_generator/
        base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        template_dir = os.path.join(base_dir, 'templates')
        static_dir = os.path.join(base_dir, 'static')

        # Set up Jinja2 environment
        env = Environment(loader=FileSystemLoader(template_dir), autoescape=True)
        template = env.get_template('check.html')

        # Render the HTML template with data
        html_string = template.render(data)

        # Load main CSS (referenced as ../static/style.css from template)
        # For WeasyPrint to find it, we need to give it the correct path or use base_url.
        # The embedded styles in check.html will be used automatically.
        # If style.css contains general styles, it's good to include it.
        css_path = os.path.join(static_dir, 'style.css')

        font_config = FontConfiguration()
        html_obj = HTML(string=html_string, base_url=template_dir) # base_url helps resolve ../static/style.css

        # Load additional CSS if needed (e.g. the main style.css)
        # WeasyPrint will pick up <link> tags if base_url is set correctly.
        # If you want to explicitly add stylesheets:
        # stylesheets = [CSS(css_path, font_config=font_config)]
        # pdf_bytes = html_obj.write_pdf(stylesheets=stylesheets, font_config=font_config)

        if output_path:
            html_obj.write_pdf(output_path)
            print(f"Check PDF generated and saved to {output_path}")
            return True # Indicate success when file is saved
        else:
            pdf_bytes = html_obj.write_pdf()
            print("Check PDF generated as bytes.")
            return pdf_bytes

    except Exception as e:
        print(f"Error generating check: {e}")
        return None

if __name__ == '__main__':
    # Example Usage (for testing purposes)
    # This assumes the script is run from within the 'app' directory or the project root is in PYTHONPATH

    # Determine project base for saving test PDF
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    sample_check_data = {
        'bank_name': 'Community Bank & Trust',
        'bank_address': '456 Finance Ave, Moneytown, ST 67890',
        'check_number': '1001',
        'date': 'November 20, 2023',
        'payee_name': 'Jane K. Smith',
        'amount_numeric': '123.45',
        'amount_words': 'ONE HUNDRED TWENTY-THREE AND 45/100',
        'memo': 'Invoice #INV789 / Services Rendered',
        'routing_number': '123456789', # Example routing
        'account_number': '9876543210' # Example account
    }

    # Generate and save to file
    output_file = os.path.join(project_root, 'generated_check_test.pdf')
    if generate_check(sample_check_data, output_path=output_file):
        print(f"Test check saved to {output_file}")
    else:
        print(f"Failed to save test check to {output_file}")

    # Generate as bytes and save
    # pdf_data_bytes = generate_check(sample_check_data)
    # if pdf_data_bytes:
    #     bytes_output_file = os.path.join(project_root, 'generated_check_bytes_test.pdf')
    #     try:
    #         with open(bytes_output_file, 'wb') as f:
    #             f.write(pdf_data_bytes)
    #         print(f"Test check from bytes saved to {bytes_output_file}")
    #     except Exception as e:
    #         print(f"Error saving bytes PDF: {e}")
    # else:
    #     print("Failed to generate test check as bytes.")
