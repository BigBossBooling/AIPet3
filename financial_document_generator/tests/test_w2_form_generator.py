import unittest
from ..app.w2_form_generator import generate_w2_form

class TestW2FormGenerator(unittest.TestCase):

    def test_generate_w2_form_smoke(self):
        # Smoke test to ensure the function runs without crashing
        sample_data = {"employee_name": "Jane Doe", "year": 2023}
        try:
            generate_w2_form(sample_data)
            ran_successfully = True
        except Exception as e:
            print(f"Test failed: {e}")
            ran_successfully = False
        self.assertTrue(ran_successfully, "generate_w2_form should run without errors.")

if __name__ == '__main__':
    unittest.main()
