import os
from jinja2 import Environment, FileSystemLoader
from weasyprint import HTML
from weasyprint.text.fonts import FontConfiguration # Corrected import

def generate_bank_statement(statement_data, output_path=None):
    '''
    Generates a bank statement PDF from data.

    Args:
        statement_data (dict): A dictionary containing all bank statement details.
                               Expected structure:
                               {
                                   'bank_details': {'name': '', 'address': '', 'phone': '', 'website': ''},
                                   'account_holder': {'name': '', 'address_line1': '', 'address_line2': ''},
                                   'account_details': {'number': '', 'type': ''},
                                   'statement_period': {'start_date': '', 'end_date': ''},
                                   'statement_date': '',
                                   'summary': {
                                       'beginning_balance': '',
                                       'total_deposits': '',
                                       'total_withdrawals': '',
                                       'ending_balance': ''
                                   },
                                   'transactions': [
                                       {'date': '', 'description': '', 'withdrawal': '', 'deposit': '', 'balance': ''},
                                       # ... more transactions
                                   ]
                               }
        output_path (str, optional): If provided, saves the PDF to this path.
                                     Otherwise, returns the PDF as bytes.

    Returns:
        bytes or bool: PDF content as bytes if output_path is None,
                       True if file is saved successfully,
                       None on error.
    '''
    try:
        base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        template_dir = os.path.join(base_dir, 'templates')

        env = Environment(loader=FileSystemLoader(template_dir), autoescape=True)
        template = env.get_template('bank_statement.html')

        html_string = template.render(statement_data)

        html_obj = HTML(string=html_string, base_url=template_dir) # base_url for resolving static/style.css

        if output_path:
            html_obj.write_pdf(output_path)
            print(f"Bank statement PDF generated and saved to {output_path}")
            return True
        else:
            pdf_bytes = html_obj.write_pdf()
            print("Bank statement PDF generated as bytes.")
            return pdf_bytes

    except Exception as e:
        print(f"Error generating bank statement: {e}")
        return None

if __name__ == '__main__':
    # Example Usage (for testing purposes)
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    sample_statement_data = {
        'bank_details': {
            'name': 'Global Finance Corp',
            'address': '1 Corporate Drive, Metropolis, USA 10001',
            'phone': '(800) 555-0100',
            'website': 'www.globalfinancecorp.com'
        },
        'account_holder': {
            'name': 'Alice B. Wonderland',
            'address_line1': '456 Looking Glass Lane',
            'address_line2': 'Fantasyland, FL 67890'
        },
        'account_details': {
            'number': 'ACCT-9876543210',
            'type': 'Premium Checking Account'
        },
        'statement_period': {
            'start_date': '10/01/2023',
            'end_date': '10/31/2023'
        },
        'statement_date': '11/05/2023',
        'summary': {
            'beginning_balance': '$5,250.75',
            'total_deposits': '$2,100.00',
            'total_withdrawals': '$850.25',
            'ending_balance': '$6,500.50'
        },
        'transactions': [
            {'date': '10/02/2023', 'description': 'Direct Deposit - Employer A', 'withdrawal': '', 'deposit': '$1,500.00', 'balance': '$6,750.75'},
            {'date': '10/05/2023', 'description': 'Online Purchase - BookStore.com', 'withdrawal': '$50.25', 'deposit': '', 'balance': '$6,700.50'},
            {'date': '10/10/2023', 'description': 'ATM Withdrawal - Branch ATM', 'withdrawal': '$200.00', 'deposit': '', 'balance': '$6,500.50'},
            {'date': '10/15/2023', 'description': 'Check #1234 Cashed', 'withdrawal': '$600.00', 'deposit': '', 'balance': '$5,900.50'},
            {'date': '10/20/2023', 'description': 'Incoming Wire Transfer - Client B', 'withdrawal': '', 'deposit': '$600.00', 'balance': '$6,500.50'},
        ]
    }

    output_file = os.path.join(project_root, 'generated_bank_statement_test.pdf')
    if generate_bank_statement(sample_statement_data, output_path=output_file):
        print(f"Test bank statement saved to {output_file}")
    else:
        print(f"Failed to save test bank statement to {output_file}")

    # To test byte generation:
    # pdf_bytes = generate_bank_statement(sample_statement_data)
    # if pdf_bytes:
    #     bytes_output_file = os.path.join(project_root, 'generated_bank_statement_bytes_test.pdf')
    #     with open(bytes_output_file, 'wb') as f:
    #         f.write(pdf_bytes)
    #     print(f"Test bank statement from bytes saved to {bytes_output_file}")
    # else:
    #     print("Failed to generate test bank statement as bytes.")
