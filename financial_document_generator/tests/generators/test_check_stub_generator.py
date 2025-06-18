import unittest
import os
import io
import datetime
from financial_document_generator.app.generators.check_stub_generator import CheckStubGenerator

class TestCheckStubGenerator(unittest.TestCase):

    def setUp(self):
        self.output_path = "test_check_stub_output.pdf"

        self.sample_data = {
            "company_name": "Innovatech Solutions Ltd.",
            "company_address": "456 Future Drive, Tech Park, CA 90210",
            "employee_name": "Alice B. Coder",
            "employee_id": "DEV78901",
            "pay_period_start": datetime.date(2023, 7, 1),
            "pay_period_end": datetime.date(2023, 7, 15),
            "pay_date": datetime.date(2023, 7, 20),
            "earnings_items": [
                {"description": "Regular Hours (80)", "amount": 4000.00},
                {"description": "Overtime Hours (10 @ 1.5x)", "amount": 750.00},
                {"description": "Project Completion Bonus", "amount": 500.00}
            ],
            "earnings_total": 5250.00,
            "deduction_items": [
                {"description": "Federal Income Tax", "amount": 800.00},
                {"description": "State Income Tax", "amount": 250.00},
                {"description": "Social Security (FICA)", "amount": 325.50},
                {"description": "Medicare (FICA)", "amount": 76.13},
                {"description": "Health Insurance Premium", "amount": 150.00},
                {"description": "401k Contribution (5%)", "amount": 262.50}
            ],
            "deductions_total": 1864.13,
            "net_pay": 3385.87,
            "ytd_gross_earnings": 75000.50,
            "ytd_deductions": 25000.25,
            "ytd_net_pay": 50000.25
        }
        self.generator = CheckStubGenerator(self.sample_data)

    def tearDown(self):
        if os.path.exists(self.output_path):
            os.remove(self.output_path)

    def test_initialization(self):
        # CheckStubGenerator's __init__ directly stores data via super().__init__()
        # and currently does not have its own _prepare_data_for_render type method.
        self.assertEqual(self.generator.data, self.sample_data)

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

        written_bytes = pdf_buffer.getvalue() # Get all bytes from the buffer
        self.assertIsNotNone(written_bytes, "Buffer should contain data.")
        self.assertIsInstance(written_bytes, bytes, "Buffered data should be bytes.")
        self.assertTrue(len(written_bytes) > 0, "PDF data should be written to the buffer (non-empty).")
        self.assertTrue(written_bytes.startswith(b'%PDF-'), "PDF content in buffer should start with PDF magic number.")

if __name__ == '__main__':
    unittest.main()
