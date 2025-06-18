import os
import pathlib
import datetime
import io # Added for io.BytesIO
from typing import Union, Optional, Any, Dict # Added for type hinting

from .base_generator import BaseGenerator # Import BaseGenerator
# Assuming Jinja2 and WeasyPrint are available as they were used by the old function
# BaseGenerator._render_template handles Jinja2, so direct imports might not be needed here
# unless for specific configurations.
from weasyprint import HTML
# from weasyprint.text.fonts import FontConfiguration # Not explicitly used in old code's WeasyPrint call directly

class CheckGenerator(BaseGenerator):
    def __init__(self, data: dict):
        super().__init__(data)
        # Specific initializations for CheckGenerator can go here if needed
        # For example, pre-processing some data fields from self.data
        self._prepare_data_for_render()

    def _prepare_data_for_render(self):
        """
        Pre-processes specific fields in self.data before rendering.
        This includes logo paths, font paths, and date formatting.
        """
        render_data = self.data # Work on a reference, changes reflect in self.data

        # --- Handle bank_logo_url ---
        if 'bank_logo_url' in render_data and \
           render_data['bank_logo_url'] and \
           not str(render_data['bank_logo_url']).startswith(('http://', 'https://', 'file://')):

            logo_path_str = str(render_data['bank_logo_url'])
            # Resolve path relative to project root if not absolute
            # Assuming self._get_project_root() is available from BaseGenerator
            logo_path_obj = pathlib.Path(logo_path_str)
            if not logo_path_obj.is_absolute():
                logo_path_obj = pathlib.Path(self._get_project_root()) / logo_path_str

            if logo_path_obj.is_file():
                render_data['bank_logo_url'] = logo_path_obj.as_uri()
            else:
                print(f"Warning: Local logo file not found: {logo_path_str} (resolved to: {logo_path_obj})")
                render_data['bank_logo_url'] = None

        # --- Process font_configs ---
        if 'font_configs' in render_data and isinstance(render_data['font_configs'], list):
            processed_font_configs = []
            for font_conf in render_data['font_configs']:
                if isinstance(font_conf, dict) and 'font_family' in font_conf and 'font_path' in font_conf:
                    font_path_str = str(font_conf['font_path'])
                    if font_path_str and not font_path_str.startswith(('http://', 'https://', 'file://')):
                        font_file_path_obj = pathlib.Path(font_path_str)
                        if not font_file_path_obj.is_absolute():
                             font_file_path_obj = pathlib.Path(self._get_project_root()) / font_path_str

                        if font_file_path_obj.is_file():
                            processed_conf = font_conf.copy()
                            processed_conf['font_src_url'] = font_file_path_obj.as_uri()
                            processed_font_configs.append(processed_conf)
                        else:
                            print(f"Warning: Font file not found: {font_path_str} (resolved to: {font_file_path_obj}) for family {font_conf['font_family']}")
                    elif font_path_str:
                        processed_conf = font_conf.copy()
                        processed_conf['font_src_url'] = font_path_str
                        processed_font_configs.append(processed_conf)
                    else:
                        print(f"Warning: Empty font_path for family {font_conf['font_family']}. Skipping.")
                else:
                    print(f"Warning: Invalid font configuration item skipped: {font_conf}")
            render_data['font_configs'] = processed_font_configs
        elif 'font_configs' in render_data:
             print(f"Warning: 'font_configs' was provided but is not a list. Ignoring. Value: {render_data['font_configs']}")
             render_data['font_configs'] = []
        else: # Ensure it exists for the template
            render_data['font_configs'] = []

        # --- Format date if date_output_format is provided and date is a datetime.date object ---
        if 'date_output_format' in render_data and render_data['date_output_format'] and \
           'date' in render_data and isinstance(render_data['date'], datetime.date):
            try:
                formatted_date = render_data['date'].strftime(str(render_data['date_output_format']))
                render_data['date_for_display'] = formatted_date # Store formatted date in a new key
            except ValueError as ve:
                print(f"Warning: Invalid date_output_format string '{render_data['date_output_format']}'. Error: {ve}")
                render_data['date_for_display'] = str(render_data['date'])
            except Exception as e:
                print(f"Warning: Could not format date using format '{render_data['date_output_format']}'. Error: {e}")
                render_data['date_for_display'] = str(render_data['date'])
        elif 'date' in render_data and render_data['date'] is not None:
             render_data['date_for_display'] = str(render_data['date'])


    def generate_pdf(self, output_path_or_buffer: Optional[Union[str, io.BytesIO]] = None) -> Optional[Union[bool, bytes]]:
        try:
            # Use _render_template from BaseGenerator.
            # The template 'check.html' should be directly under the 'templates' folder.
            html_string = self._render_template('check.html', self.data)

            # Use project root as base_url for WeasyPrint to correctly find static assets if linked directly
            # e.g. if template used <img src="/static/images/logo.png">
            # The current check.html does not seem to use such paths, relies on data for logo.
            html_obj = HTML(string=html_string, base_url=self._get_project_root())

            is_path = isinstance(output_path_or_buffer, str)
            is_buffer = isinstance(output_path_or_buffer, io.BytesIO)

            if is_path:
                html_obj.write_pdf(output_path_or_buffer)
                print(f"Check PDF generated and saved to {output_path_or_buffer}")
                return True
            elif is_buffer:
                html_obj.write_pdf(target=output_path_or_buffer)
                # Buffer is modified in-place, caller should handle it.
                return True # Indicate success
            else: # None, return bytes
                pdf_bytes = html_obj.write_pdf()
                print("Check PDF generated as bytes.")
                return pdf_bytes

        except Exception as e:
            print(f"Error generating check PDF: {e}")
            # import traceback
            # traceback.print_exc() # For detailed debugging
            return None if output_path_or_buffer is None else False


# The old generate_check function and its __main__ block are commented out or removed.
# '''
# def generate_check(data, output_path=None):
#     # ... old code ...
# if __name__ == '__main__':
#     # ... old main block ...
# '''
