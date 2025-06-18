import unittest
import os
import io
import datetime
from financial_document_generator.app.generators.earning_statement_generator import EarningStatementGenerator

class TestEarningStatementGenerator(unittest.TestCase):

    def setUp(self):
        self.output_path = "test_earning_statement_output.pdf"

        self.sample_data = {
            "company_name": "Synergy Corp",
            "company_address": "789 Innovation Drive, Future City, FC 67890",
            "employee_name": "Jane Q. Developer",
            "employee_id": "EMP12345",
            "pay_period_start": datetime.date(2023, 9, 1),
            "pay_period_end": datetime.date(2023, 9, 15),
            "pay_date": datetime.date(2023, 9, 20),
            "earnings_items": [
                {"description": "Salary", "amount": 2500.00},
                {"description": "Performance Bonus", "amount": 500.00}
            ],
            # earnings_total will be calculated by _prepare_data (2500 + 500 = 3000)
            "deduction_items": [
                {"description": "Federal Tax", "amount": 400.00},
                {"description": "State Tax", "amount": 150.00},
                {"description": "Health Insurance", "amount": 100.00}
            ],
            # deductions_total will be calculated by _prepare_data (400 + 150 + 100 = 650)
            # net_pay will be calculated by _prepare_data (3000 - 650 = 2350)
            "ytd_gross_earnings": 50000.00,
            "ytd_deductions": 12000.00,
            "ytd_net_pay": 38000.00,
            "corporate_notes": "Thank you for your continued dedication and hard work! Don't forget the upcoming company picnic on the 30th."
        }
        # Make a deep copy for the generator to modify if needed, keeping original for comparison
        self.generator = EarningStatementGenerator(self.sample_data.copy())

    def tearDown(self):
        if os.path.exists(self.output_path):
            os.remove(self.output_path)

    def test_initialization_and_data_preparation(self):
        # Test if initial data is stored and _prepare_data works
        self.assertEqual(self.generator.data['company_name'], self.sample_data['company_name'])
        self.assertEqual(self.generator.data['corporate_notes'], self.sample_data['corporate_notes'])

        # Test calculated totals
        self.assertAlmostEqual(self.generator.data['earnings_total'], 3000.00, places=2)
        self.assertAlmostEqual(self.generator.data['deductions_total'], 650.00, places=2)
        self.assertAlmostEqual(self.generator.data['net_pay'], 2350.00, places=2)

        # Test that item lists are processed (amounts should be floats)
        self.assertIsInstance(self.generator.data['earnings_items'][0]['amount'], float)
        self.assertIsInstance(self.generator.data['deduction_items'][0]['amount'], float)


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
