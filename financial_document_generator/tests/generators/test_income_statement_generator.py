import unittest
import os
import io
# import datetime # Not strictly needed for sample data if period_description is a string
from financial_document_generator.app.generators.income_statement_generator import IncomeStatementGenerator

class TestIncomeStatementGenerator(unittest.TestCase):

    def setUp(self):
        self.output_path = "test_income_statement_output.pdf"

        self.common_header = {
            'company_name': 'TestCorp Inc.',
            'report_title': 'Consolidated Statement of Income',
            'period_description': 'For the Fiscal Year Ended December 31, 2023'
        }

        self.sample_data_basic = {
            **self.common_header,
            'revenues_total': 10000.00,
            'cogs_total': 4000.00,
            # Gross Profit should be 6000.00
            'operating_expenses_total': 2500.00,
            # Operating Income should be 3500.00
            'other_income_total': 500.00, # e.g., Interest Income
            # Income Before Tax should be 4000.00
            'income_tax_expense': 800.00,
            # Net Income should be 3200.00
            # Item lists are empty or not provided
            'revenue_items': [],
            'cogs_items': [],
            'operating_expense_items': [],
            'other_income_expense_items': []
        }

        self.sample_data_itemized = {
            **self.common_header,
            'revenue_items': [
                {'description': 'Product Sales', 'amount': 18000.00},
                {'description': 'Service Revenue', 'amount': 2000.00}
            ], # revenues_total should be 20000.00
            'cogs_items': [
                {'description': 'Material Costs', 'amount': 5000.00},
                {'description': 'Direct Labor', 'amount': 3000.00}
            ], # cogs_total should be 8000.00
            # Gross Profit should be 12000.00
            'operating_expense_items': [
                {'description': 'Sales & Marketing', 'amount': 2000.00},
                {'description': 'Research & Development', 'amount': 1500.00},
                {'description': 'General & Administrative', 'amount': 2500.00}
            ], # operating_expenses_total should be 6000.00
            # Operating Income should be 6000.00
            'other_income_expense_items': [
                {'description': 'Interest Income', 'amount': 300.00},
                {'description': 'Gain on Asset Sale', 'amount': 700.00}
            ], # other_income_total should be 1000.00
            # Income Before Tax should be 7000.00
            'income_tax_expense': 1400.00, # Provided directly
            # Net Income should be 5600.00
            # Totals that can be derived are omitted to test calculation
        }

        # Sample data to test fallback for individual operating expenses (if items list is empty)
        self.sample_data_individual_op_expenses = {
            **self.common_header,
            'revenues_total': 5000.00,
            'cogs_total': 2000.00, # GP = 3000
            'operating_expense_items': [], # Empty list
            # Provide individual op expenses instead
            'selling_expenses': 500.00,
            'general_administrative_expenses': 700.00,
            'research_development_expenses': 300.00,
            # operating_expenses_total should be 500+700+300 = 1500
            # Operating Income should be 3000 - 1500 = 1500
            'other_income_total': 100.00, # IBT = 1600
            'income_tax_expense': 320.00, # NI = 1280
        }


    def tearDown(self):
        if os.path.exists(self.output_path):
            os.remove(self.output_path)

    def test_initialization_and_calculations_basic(self):
        generator = IncomeStatementGenerator(self.sample_data_basic.copy()) # Use copy to avoid modification across tests
        # Test that data preparation and calculations are done in __init__
        self.assertAlmostEqual(generator.data['gross_profit'], 6000.00, places=2)
        self.assertAlmostEqual(generator.data['operating_income'], 3500.00, places=2)
        self.assertAlmostEqual(generator.data['income_before_tax'], 4000.00, places=2)
        self.assertAlmostEqual(generator.data['net_income'], 3200.00, places=2)

    def test_initialization_and_calculations_itemized(self):
        generator = IncomeStatementGenerator(self.sample_data_itemized.copy())
        # Test totals calculated from items
        self.assertAlmostEqual(generator.data['revenues_total'], 20000.00, places=2)
        self.assertAlmostEqual(generator.data['cogs_total'], 8000.00, places=2)
        self.assertAlmostEqual(generator.data['operating_expenses_total'], 6000.00, places=2)
        self.assertAlmostEqual(generator.data['other_income_total'], 1000.00, places=2)
        # Test final derived values
        self.assertAlmostEqual(generator.data['gross_profit'], 12000.00, places=2)
        self.assertAlmostEqual(generator.data['operating_income'], 6000.00, places=2)
        self.assertAlmostEqual(generator.data['income_before_tax'], 7000.00, places=2)
        self.assertAlmostEqual(generator.data['net_income'], 5600.00, places=2)

    def test_initialization_with_individual_op_expenses(self):
        generator = IncomeStatementGenerator(self.sample_data_individual_op_expenses.copy())
        # Check if individual op expenses are summed up if operating_expense_items is empty
        # and operating_expenses_total is not provided (or also needs calculation).
        # The _prepare_and_calculate_data sums items first. If item list is empty,
        # and total is not provided, it *doesn't* currently sum individual fields.
        # This test will verify current behavior.
        # The template *does* have a fallback for individual fields.
        # Let's adjust _prepare_and_calculate_data if we want it to sum these.
        # For now, it ensures they are numeric if op_expense_items is empty.
        self.assertEqual(generator.data['selling_expenses'], 500.00)
        self.assertEqual(generator.data['general_administrative_expenses'], 700.00)
        self.assertEqual(generator.data['research_development_expenses'], 300.00)

        # If operating_expenses_total was NOT provided, and items ARE empty, it should become sum of individuals
        # This requires modification in _prepare_and_calculate_data
        # Current _prepare_and_calculate_data:
        # if not self.data.get('operating_expenses_total') and self.data.get('operating_expense_items'):
        #    self.data['operating_expenses_total'] = sum(...)
        # This means if items is empty, it won't sum individuals into the total.
        # The template handles displaying individuals if item list is empty.
        # The _prepare_and_calculate_data should calculate operating_expenses_total from individuals
        # if items are empty AND total is not provided.
        # Let's assume for now the generator ensures these are numbers and the template displays them.
        # The final calculation for operating_income etc. relies on operating_expenses_total.
        # If operating_expenses_total is not provided, it defaults to 0 unless items exist.
        # So, need to ensure operating_expenses_total is calculated if items are empty but individuals are present.
        # The current generator logic might make operating_expenses_total 0.0 if items are empty and total not given.
        # This is a good point to refine the generator logic or clarify test expectation.
        # For now, let's test the final calculations based on this assumption.
        # If op_exp_total becomes 0, then op_income = 3000 - 0 = 3000.
        # Let's assume the generator is smart enough to sum these if total is not given.
        # (After reviewing IncomeStatementGenerator, it *doesn't* sum individual fields into total if items are empty)
        # So, if operating_expenses_total is not in sample_data_individual_op_expenses, it will be 0.
        # Then op_income = 3000 - 0 = 3000. IBT = 3000 + 100 = 3100. NI = 3100 - 320 = 2780.
        # This is how it currently works. The template would show the itemized, but total would be 0.
        # This is not ideal. The generator SHOULD sum these if items are empty and total is not provided.
        # I will proceed assuming the generator *will* be fixed/is smart.
        # Expected: op_exp_total = 1500. op_income = 1500. IBT = 1600. NI = 1280.
        # This means the test will fail until generator logic is updated.
        # For now, I will write the test for the *desired* behavior.
        self.assertAlmostEqual(generator.data.get('operating_expenses_total', 0.0), 1500.00, places=2, msg="Total OpEx from individuals if items empty")
        self.assertAlmostEqual(generator.data['gross_profit'], 3000.00, places=2)
        self.assertAlmostEqual(generator.data['operating_income'], 1500.00, places=2)
        self.assertAlmostEqual(generator.data['income_before_tax'], 1600.00, places=2)
        self.assertAlmostEqual(generator.data['net_income'], 1280.00, places=2)


    def test_generate_pdf_returns_bytes(self):
        generator = IncomeStatementGenerator(self.sample_data_basic.copy())
        pdf_bytes = generator.generate_pdf()
        self.assertIsNotNone(pdf_bytes, "generate_pdf() should return bytes.")
        self.assertIsInstance(pdf_bytes, bytes, "generate_pdf() should return a bytes object.")
        self.assertTrue(len(pdf_bytes) > 0, "Returned PDF bytes should not be empty.")
        self.assertTrue(pdf_bytes.startswith(b'%PDF-'), "Returned bytes should start with PDF magic number.")

    def test_generate_pdf_saves_to_file(self):
        generator = IncomeStatementGenerator(self.sample_data_basic.copy())
        result = generator.generate_pdf(output_path_or_buffer=self.output_path)
        self.assertTrue(result, "generate_pdf should return True on successful file save.")
        self.assertTrue(os.path.exists(self.output_path), "PDF file should be created.")
        self.assertTrue(os.path.getsize(self.output_path) > 0, "Generated PDF file should not be empty.")

    def test_generate_pdf_writes_to_buffer(self):
        generator = IncomeStatementGenerator(self.sample_data_basic.copy())
        pdf_buffer = io.BytesIO()
        result = generator.generate_pdf(output_path_or_buffer=pdf_buffer)
        self.assertTrue(result, "generate_pdf should return True on successful buffer write.")

        written_bytes = pdf_buffer.getvalue()
        self.assertIsNotNone(written_bytes, "Buffer should contain data.")
        self.assertIsInstance(written_bytes, bytes, "Buffered data should be bytes.")
        self.assertTrue(len(written_bytes) > 0, "PDF data in buffer should not be empty.")
        self.assertTrue(written_bytes.startswith(b'%PDF-'), "PDF content in buffer should start with PDF magic number.")

if __name__ == '__main__':
    unittest.main()
