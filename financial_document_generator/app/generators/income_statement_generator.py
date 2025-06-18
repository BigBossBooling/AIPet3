import os
import io
from typing import Union, Optional, Dict, List, Any

from .base_generator import BaseGenerator
from weasyprint import HTML
from weasyprint.fonts import FontConfiguration

class IncomeStatementGenerator(BaseGenerator):
    def __init__(self, data: Dict):
        """
        Initializes IncomeStatementGenerator with income statement data.
        Args:
            data (dict): A dictionary containing income statement details.
        """
        super().__init__(data)
        self._prepare_and_calculate_data()

    def _ensure_numeric(self, keys: List[str], default_value: float = 0.0) -> None:
        """Ensure specified keys in self.data are numeric (float)."""
        for key in keys:
            try:
                self.data[key] = float(self.data.get(key, default_value))
            except (ValueError, TypeError):
                self.data[key] = default_value

    def _ensure_list_of_dicts(self, keys: List[str]) -> None:
        """Ensure specified keys in self.data are lists of dictionaries, with numeric amounts."""
        for key in keys:
            items = self.data.get(key)
            if not isinstance(items, list):
                self.data[key] = []
                continue

            processed_items = []
            for item in items:
                if isinstance(item, dict) and 'amount' in item:
                    try:
                        item['amount'] = float(item.get('amount', 0.0))
                    except (ValueError, TypeError):
                        item['amount'] = 0.0
                    processed_items.append(item)
                # else, skip malformed item
            self.data[key] = processed_items


    def _prepare_and_calculate_data(self) -> None:
        """
        Prepares data and calculates derived fields for the income statement.
        Ensures all necessary numeric fields are floats and item lists are correctly structured.
        """
        # Ensure numeric fields for totals are floats
        numeric_totals = [
            'revenues_total', 'cogs_total', 'gross_profit',
            'operating_expenses_total', 'operating_income',
            'other_income_total', 'income_before_tax',
            'income_tax_expense', 'net_income'
        ]
        self._ensure_numeric(numeric_totals)

        # Ensure item lists are lists of dicts with numeric amounts
        item_lists_keys = [
            'revenue_items', 'cogs_items',
            'operating_expense_items', 'other_income_expense_items'
        ]
        self._ensure_list_of_dicts(item_lists_keys)

        # Calculate totals if not provided but item lists are
        if not self.data.get('revenues_total') and self.data.get('revenue_items'):
            self.data['revenues_total'] = sum(item.get('amount', 0.0) for item in self.data['revenue_items'])

        if not self.data.get('cogs_total') and self.data.get('cogs_items'):
            self.data['cogs_total'] = sum(item.get('amount', 0.0) for item in self.data['cogs_items'])

        if not self.data.get('operating_expenses_total') and self.data.get('operating_expense_items'):
            self.data['operating_expenses_total'] = sum(item.get('amount', 0.0) for item in self.data['operating_expense_items'])

        if not self.data.get('other_income_total') and self.data.get('other_income_expense_items'):
            self.data['other_income_total'] = sum(item.get('amount', 0.0) for item in self.data['other_income_expense_items'])

        # Calculate derived fields if not provided directly
        # Gross Profit = Total Revenues - Total COGS
        if 'gross_profit' not in self.data or not self.data['gross_profit']: # check if zero or not present
            self.data['gross_profit'] = self.data['revenues_total'] - self.data['cogs_total']

        # Operating Income = Gross Profit - Total Operating Expenses
        if 'operating_income' not in self.data or not self.data['operating_income']:
            self.data['operating_income'] = self.data['gross_profit'] - self.data['operating_expenses_total']

        # Income Before Tax = Operating Income + Total Other Income/Expenses
        if 'income_before_tax' not in self.data or not self.data['income_before_tax']:
            self.data['income_before_tax'] = self.data['operating_income'] + self.data['other_income_total']

        # Net Income = Income Before Tax - Income Tax Expense
        if 'net_income' not in self.data or not self.data['net_income']:
            self.data['net_income'] = self.data['income_before_tax'] - self.data['income_tax_expense']

        # Ensure default empty lists for items if not provided, for template rendering
        for key in item_lists_keys:
            if key not in self.data:
                self.data[key] = []

        # Fallback for individual operating expense items if operating_expense_items list is empty
        # These are used by the template if operating_expense_items is not provided or empty.
        if not self.data.get('operating_expense_items'):
            self._ensure_numeric(['selling_expenses', 'general_administrative_expenses', 'research_development_expenses'])


    def generate_pdf(self, output_path_or_buffer: Optional[Union[str, io.BytesIO]] = None) -> Optional[Union[bool, bytes]]:
        try:
            rendered_html = self._render_template('income_statement.html', self.data)
            font_config = FontConfiguration()
            html_doc = HTML(string=rendered_html, base_url=self._get_project_root())

            is_path = isinstance(output_path_or_buffer, str)
            is_buffer = isinstance(output_path_or_buffer, io.BytesIO)

            if is_path:
                html_doc.write_pdf(output_path_or_buffer, font_config=font_config)
                return True
            elif is_buffer:
                html_doc.write_pdf(target=output_path_or_buffer, font_config=font_config)
                return True
            else: # None, return bytes
                pdf_bytes = html_doc.write_pdf(font_config=font_config)
                return pdf_bytes
        except Exception as e:
            print(f"Error generating income statement PDF: {e}")
            # import traceback
            # traceback.print_exc()
            return None if output_path_or_buffer is None else False

# '''
# def generate_income_statement(data):
#     # TODO: Implement income statement generation logic
#     print(f"Generating income statement with data: {data}")
#     return "path/to/generated/income_statement.pdf" # Placeholder
# '''
