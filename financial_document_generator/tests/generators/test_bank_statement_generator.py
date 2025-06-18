import unittest
import os
import io
import datetime
from financial_document_generator.app.generators.bank_statement_generator import BankStatementGenerator

class TestBankStatementGenerator(unittest.TestCase):

    def setUp(self):
        self.output_path = "test_bank_statement_output.pdf"

        self.sample_data = {
            'bank_details': {
                'name': 'Secure Trust Bank',
                'address': '789 Vault Plaza, Safe City, SC 67890',
                'phone': '(800) 123-SAFE',
                'website': 'www.securetrust.example.com'
            },
            'account_holder': {
                'name': 'Michael T. Secure',
                'address_line1': '1 Secure Way, Apt B',
                'address_line2': 'Fort Knox, KY 40121'
            },
            'account_details': {
                'number': 'ACCT-000-111-222',
                'type': 'Gold Standard Savings'
            },
            'statement_period': {
                # main.py converts form date strings to datetime.date objects
                'start_date': datetime.date(2023, 8, 1),
                'end_date': datetime.date(2023, 8, 31)
            },
            'statement_date': datetime.date(2023, 9, 5), # As datetime.date object
            'summary': {
                # These are typically strings in templates, matching form input style
                'beginning_balance': '10500.00',
                'total_deposits': '3200.50',
                'total_withdrawals': '1800.25',
                'ending_balance': '11900.25'
            },
            'transactions': [
                {
                    'date': datetime.date(2023, 8, 5), # As datetime.date object
                    'description': 'Salary Deposit - SecureCorp',
                    'withdrawal': '',
                    'deposit': '3000.00',
                    'balance': '13500.00'
                },
                {
                    'date': datetime.date(2023, 8, 10),
                    'description': 'Utility Payment - SafeEnergy',
                    'withdrawal': '150.25',
                    'deposit': '',
                    'balance': '13349.75'
                },
                {
                    'date': datetime.date(2023, 8, 15),
                    'description': 'Online Transfer to Savings #002',
                    'withdrawal': '1000.00',
                    'deposit': '',
                    'balance': '12349.75'
                },
                {
                    'date': datetime.date(2023, 8, 25),
                    'description': 'Interest Earned',
                    'withdrawal': '',
                    'deposit': '200.50',
                    'balance': '12550.25' # Example, actual balance may vary based on previous
                },
                 {
                    'date': datetime.date(2023, 8, 28),
                    'description': 'Service Fee',
                    'withdrawal': '50.00',
                    'deposit': '',
                    'balance': '11900.25' # Example, matching ending_balance for simplicity
                }
            ]
        }
        self.generator = BankStatementGenerator(self.sample_data)

    def tearDown(self):
        if os.path.exists(self.output_path):
            os.remove(self.output_path)

    def test_initialization(self):
        # BankStatementGenerator's __init__ directly stores data via super().__init__()
        # It does not currently have its own _prepare_data_for_render method.
        self.assertEqual(self.generator.data, self.sample_data)
        # A more robust test could check a few key nested values:
        self.assertEqual(self.generator.data['bank_details']['name'], 'Secure Trust Bank')
        self.assertEqual(len(self.generator.data['transactions']), 5)

    def test_generate_pdf_returns_bytes(self):
        pdf_bytes = self.generator.generate_pdf() # No argument
        self.assertIsNotNone(pdf_bytes, "generate_pdf() should return bytes, not None.")
        self.assertIsInstance(pdf_bytes, bytes, "generate_pdf() should return a bytes object.")
        self.assertTrue(len(pdf_bytes) > 0, "Returned PDF bytes should not be empty.")
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'), "Returned bytes should start with PDF magic number.")

    def test_generate_pdf_saves_to_file(self):
        result = self.generator.generate_pdf(output_path_or_buffer=self.output_path)
        self.assertTrue(result, "generate_pdf should return True on successful file save.")
        self.assertTrue(os.path.exists(self.output_path), "PDF file should be created at the specified path.")
        self.assertTrue(os.path.getsize(self.output_path) > 0, "Generated PDF file should not be empty.")

    def test_generate_pdf_writes_to_buffer(self):
        pdf_buffer = io.BytesIO()
        result = self.generator.generate_pdf(output_path_or_buffer=pdf_buffer)
        self.assertTrue(result, "generate_pdf should return True on successful buffer write.")

        written_bytes = pdf_buffer.getvalue()
        self.assertIsNotNone(written_bytes, "Buffer should contain data.")
        self.assertIsInstance(written_bytes, bytes, "Buffered data should be bytes.")
        self.assertTrue(len(written_bytes) > 0, "PDF data should be written to the buffer (non-empty).")
        self.assertTrue(written_bytes.startswith(b'%PDF-'), "PDF content in buffer should start with PDF magic number.")

if __name__ == '__main__':
    unittest.main()
