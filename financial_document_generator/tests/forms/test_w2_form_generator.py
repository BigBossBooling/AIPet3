import unittest
import os
from financial_document_generator.app.forms.w2_form_generator import W2Form

class TestW2Form(unittest.TestCase):

    def setUp(self):
        self.sample_data = {
            "employee_name": "John Doe",
            "employee_ssn": "123-45-6789",
            "employer_name": "Acme Corp",
            "employer_ein": "98-7654321",
            "wages_tips_other_compensation": 60000.50,
            "federal_income_tax_withheld": 12000.75,
            "social_security_wages": 60000.50,
            "medicare_wages_and_tips": 60000.50,
            "social_security_tax_withheld": 3720.03,
            "medicare_tax_withheld": 870.01,
            "state_employer_state_id_no": "CA123456",
            "state_wages_tips_etc": 60000.50,
            "state_income_tax": 3000.25,
            "local_wages_tips_etc": 0.0,
            "local_income_tax": 0.0,
            "locality_name": ""
        }
        self.w2_form_instance = W2Form(**self.sample_data)
        self.temp_pdf_path = "temp_test_w2_form.pdf" # Will be created in the repo root

    def tearDown(self):
        if os.path.exists(self.temp_pdf_path):
            os.remove(self.temp_pdf_path)

    def test_create_w2form_instance(self):
        self.assertIsInstance(self.w2_form_instance, W2Form)
        for key, value in self.sample_data.items():
            self.assertEqual(getattr(self.w2_form_instance, key), value)

    def test_w2form_to_dict_and_from_dict(self):
        form_dict = self.w2_form_instance.to_dict()
        self.assertIsInstance(form_dict, dict)

        # Check if all original keys are in the dictionary
        for key in self.sample_data.keys():
            self.assertIn(key, form_dict)
            self.assertEqual(form_dict[key], self.sample_data[key])

        new_w2_instance = W2Form.from_dict(form_dict)
        self.assertIsInstance(new_w2_instance, W2Form)

        for key in self.sample_data.keys():
            self.assertEqual(getattr(new_w2_instance, key), self.sample_data[key])

        # Also check if the two instances are equal by their dicts
        self.assertEqual(new_w2_instance.to_dict(), self.w2_form_instance.to_dict())

    def test_generate_pdf_creates_file(self):
        # Ensure the file does not exist before test
        if os.path.exists(self.temp_pdf_path):
            os.remove(self.temp_pdf_path)

        self.w2_form_instance.generate_pdf(self.temp_pdf_path)
        self.assertTrue(os.path.exists(self.temp_pdf_path), f"PDF file was not created at {self.temp_pdf_path}")
        self.assertTrue(os.path.getsize(self.temp_pdf_path) > 0, "PDF file is empty")

if __name__ == '__main__':
    unittest.main()
