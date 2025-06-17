import unittest
from ..app.check_stub_generator import generate_check_stub

class TestCheckStubGenerator(unittest.TestCase):

    def test_generate_check_stub_smoke(self):
        # Smoke test to ensure the function runs without crashing
        sample_data = {"employee_name": "Jane Doe", "hours_worked": 40}
        try:
            generate_check_stub(sample_data)
            ran_successfully = True
        except Exception as e:
            print(f"Test failed: {e}")
            ran_successfully = False
        self.assertTrue(ran_successfully, "generate_check_stub should run without errors.")

if __name__ == '__main__':
    unittest.main()
