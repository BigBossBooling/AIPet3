import unittest
import os
# Adjust import path to correctly locate the module
from ..app.generators.bank_statement_generator import generate_bank_statement

class TestBankStatementGenerator(unittest.TestCase):

    def setUp(self):
        # Comprehensive sample data for generating a bank statement
        self.sample_statement_data = {
            'bank_details': {
                'name': 'Test Bank International',
                'address': '1 Test Plaza, Test City, TST 10000',
                'phone': '(888) TEST-BANK',
                'website': 'www.testbankint.com'
            },
            'account_holder': {
                'name': 'Dr. Evelyn Tester',
                'address_line1': '789 Verification Rd.',
                'address_line2': 'Suite 200, Validatonia, TST 10020'
            },
            'account_details': {
                'number': 'TST-0123456789',
                'type': 'Elite Testing Account'
            },
            'statement_period': {
                'start_date': '11/01/2023',
                'end_date': '11/30/2023'
            },
            'statement_date': '12/01/2023',
            'summary': {
                'beginning_balance': '$10,000.00',
                'total_deposits': '$5,000.00',
                'total_withdrawals': '$2,500.00',
                'ending_balance': '$12,500.00'
            },
            'transactions': [
                {'date': '11/05/2023', 'description': 'Salary Deposit', 'withdrawal': '', 'deposit': '$4,000.00', 'balance': '$14,000.00'},
                {'date': '11/10/2023', 'description': 'Mortgage Payment - Test Bank', 'withdrawal': '$2,000.00', 'deposit': '', 'balance': '$12,000.00'},
                {'date': '11/15/2023', 'description': 'Online Shopping - TestRetailer', 'withdrawal': '$300.00', 'deposit': '', 'balance': '$11,700.00'},
                {'date': '11/20/2023', 'description': 'Dividend Payment', 'withdrawal': '', 'deposit': '$1,000.00', 'balance': '$12,700.00'},
                {'date': '11/25/2023', 'description': 'Utility Bill - Power Co.', 'withdrawal': '$200.00', 'deposit': '', 'balance': '$12,500.00'}
            ]
        }
        self.test_output_dir = os.path.join(os.path.dirname(__file__), 'test_outputs')
        self.test_pdf_path = os.path.join(self.test_output_dir, 'test_bank_statement.pdf')

        if not os.path.exists(self.test_output_dir):
            os.makedirs(self.test_output_dir)

    def tearDown(self):
        if os.path.exists(self.test_pdf_path):
            os.remove(self.test_pdf_path)
        if os.path.exists(self.test_output_dir) and not os.listdir(self.test_output_dir):
            os.rmdir(self.test_output_dir)

    def test_generate_bank_statement_creates_pdf_file(self):
        '''Test that generate_bank_statement creates a PDF file when output_path is provided.'''
        success = generate_bank_statement(self.sample_statement_data, output_path=self.test_pdf_path)
        self.assertTrue(success, "generate_bank_statement should return True on successful file generation.")
        self.assertTrue(os.path.exists(self.test_pdf_path), "PDF file should be created.")
        if os.path.exists(self.test_pdf_path):
            self.assertTrue(os.path.getsize(self.test_pdf_path) > 0, "Generated PDF file should not be empty.")

    def test_generate_bank_statement_returns_pdf_bytes(self):
        '''Test that generate_bank_statement returns PDF content as bytes when output_path is None.'''
        pdf_bytes = generate_bank_statement(self.sample_statement_data)
        self.assertIsNotNone(pdf_bytes, "generate_bank_statement should return PDF bytes.")
        self.assertIsInstance(pdf_bytes, bytes, "Should return a bytes object.")
        self.assertTrue(len(pdf_bytes) > 0, "Returned PDF bytes should not be empty.")
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'), "Bytes should start with PDF header '%PDF-'.")

    def test_generate_bank_statement_with_no_transactions(self):
        '''Test generating a statement with no transactions.'''
        data_no_transactions = self.sample_statement_data.copy()
        data_no_transactions['transactions'] = []
        data_no_transactions['summary']['total_deposits'] = '$0.00'
        data_no_transactions['summary']['total_withdrawals'] = '$0.00'
        data_no_transactions['summary']['ending_balance'] = data_no_transactions['summary']['beginning_balance']

        pdf_bytes = generate_bank_statement(data_no_transactions)
        self.assertIsNotNone(pdf_bytes)
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'), "PDF with no transactions should still be valid.")
        # Further checks could involve trying to extract text and look for "No transactions" message

    def test_generate_bank_statement_handles_missing_optional_data(self):
        '''Test how generator handles missing non-critical data (relies on template defaults).'''
        # Create data with some optional fields missing, e.g. bank website or account_holder.address_line2
        partial_data = self.sample_statement_data.copy()
        del partial_data['bank_details']['website']
        # Ensure essential structure for transactions is present, even if empty, if transactions are expected
        # For this test, we'll assume the template handles missing optional fields gracefully with defaults.

        pdf_bytes = generate_bank_statement(partial_data)
        self.assertIsNotNone(pdf_bytes, "generate_bank_statement should handle missing optional data using template defaults.")
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'), "PDF with partial data should still be valid.")

if __name__ == '__main__':
    unittest.main()
