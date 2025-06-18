import unittest
import os
import io
import datetime
from financial_document_generator.app.generators.check_generator import CheckGenerator

class TestCheckGenerator(unittest.TestCase):

    def setUp(self):
        self.output_path = "test_check_output.pdf"
        # Sample data for CheckGenerator
        self.sample_data = {
            'bank_name': 'Test Fidelity Bank',
            'bank_address': '100 Main Street, Testville, TS 12345',
            'check_number': '1001',
            'date': datetime.date(2023, 10, 26), # datetime.date object
            'payee_name': 'John Doe Test Services',
            'amount_numeric': '1250.75', # String, as typically from form
            'amount_words': 'ONE THOUSAND TWO HUNDRED FIFTY AND 75/100',
            'memo': 'Test Payment for Services Rendered',
            'routing_number': '123456789',
            'account_number': '9876543210',
            'bank_logo_url': None, # Optional
            'font_configs': [],    # Optional
            'date_output_format': '%m/%d/%Y' # Optional, but good for testing date prep
        }
        self.generator = CheckGenerator(self.sample_data)

    def tearDown(self):
        if os.path.exists(self.output_path):
            os.remove(self.output_path)

    def test_initialization_and_data_preparation(self):
        # Test if the initial data is stored
        self.assertEqual(self.generator.data['bank_name'], self.sample_data['bank_name'])
        self.assertEqual(self.generator.data['payee_name'], self.sample_data['payee_name'])

        # Test if _prepare_data_for_render processed the date correctly
        # Given date_output_format: '%m/%d/%Y' and date: datetime.date(2023, 10, 26)
        # it should create 'date_for_display' as '10/26/2023'
        self.assertIn('date_for_display', self.generator.data)
        self.assertEqual(self.generator.data['date_for_display'], '10/26/2023')

        # Test if font_configs is initialized if not present (it is in sample_data as [])
        self.assertIn('font_configs', self.generator.data)
        self.assertEqual(self.generator.data['font_configs'], [])


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
        self.assertTrue(pdf_buffer.getbuffer().nbytes > 0, "PDF data should be written to the buffer.")
        pdf_buffer.seek(0) # Important for reading content from start
        self.assertTrue(pdf_buffer.read().startswith(b'%PDF-'), "PDF content in buffer should start with PDF magic number.")

    def test_generate_pdf_with_local_logo(self):
        # Create a dummy logo file for this test
        dummy_logo_path = "dummy_logo_for_check_test.png"
        project_root = self.generator._get_project_root() # Get project root via generator
        full_dummy_logo_path = os.path.join(project_root, dummy_logo_path)

        with open(full_dummy_logo_path, "wb") as f: # create a tiny valid png
             f.write(b'\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89\x00\x00\x00\nIDATx\x9cc\x00\x01\x00\x00\x05\x00\x01\r\n-\xb4\x00\x00\x00\x00IEND\xaeB`\x82')

        self.addCleanup(os.remove, full_dummy_logo_path) # Ensure cleanup

        data_with_logo = self.sample_data.copy()
        data_with_logo['bank_logo_url'] = dummy_logo_path # Relative path from project root

        generator_with_logo = CheckGenerator(data_with_logo)
        pdf_bytes = generator_with_logo.generate_pdf()

        self.assertIsNotNone(pdf_bytes)
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'))
        # More specific assertion would require parsing PDF or image detection logic

if __name__ == '__main__':
    unittest.main()
