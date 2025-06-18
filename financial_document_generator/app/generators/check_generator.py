import os
import pathlib
import datetime # Import datetime
from jinja2 import Environment, FileSystemLoader
from weasyprint import HTML
from weasyprint.text.fonts import FontConfiguration # Not directly used here but good for consistency

def generate_check(data, output_path=None):
    '''
    Generates a check PDF from data.

    Args:
        data (dict): A dictionary containing check details.
                     Expected keys: bank_name, bank_address, check_number,
                                      payee_name, amount_numeric, amount_words, memo,
                                      routing_number, account_number,
                                      bank_logo_url (optional, URL or absolute local path),
                                      font_configs (optional, list of dicts:
                                          [{'font_family': 'Family Name', 'font_path': '/abs/path/to/font.ttf', 'use': 'optional_tag'}, ...]),
                                      date (str or datetime.date object),
                                      date_output_format (optional, str like "%Y-%m-%d";
                                                          if provided, 'date' should be a datetime.date object).
        output_path (str, optional): If provided, saves the PDF to this path.
                                     Otherwise, returns the PDF as bytes.

    Returns:
        bytes or bool: PDF content as bytes if output_path is None,
                       True if file is saved successfully,
                       None on error.
    '''
    try:
        render_data = data.copy()

        # --- Handle bank_logo_url ---
        if 'bank_logo_url' in render_data and \
           render_data['bank_logo_url'] and \
           not render_data['bank_logo_url'].startswith(('http://', 'https://', 'file://')):

            logo_path_str = render_data['bank_logo_url']
            logo_path_obj = pathlib.Path(logo_path_str)
            if logo_path_obj.is_file():
                render_data['bank_logo_url'] = logo_path_obj.as_uri()
            else:
                print(f"Warning: Local logo file not found: {logo_path_str} (resolved to: {logo_path_obj.resolve() if not logo_path_obj.is_absolute() else logo_path_obj})")
                render_data['bank_logo_url'] = None

        # --- Process font_configs ---
        if 'font_configs' in render_data and isinstance(render_data['font_configs'], list):
            processed_font_configs = []
            for font_conf in render_data['font_configs']:
                if isinstance(font_conf, dict) and 'font_family' in font_conf and 'font_path' in font_conf:
                    font_path_str = font_conf['font_path']
                    if font_path_str and not font_path_str.startswith(('http://', 'https://', 'file://')):
                        font_file_path_obj = pathlib.Path(font_path_str)
                        if font_file_path_obj.is_file():
                            processed_conf = font_conf.copy()
                            processed_conf['font_src_url'] = font_file_path_obj.as_uri()
                            processed_font_configs.append(processed_conf)
                        else:
                            print(f"Warning: Font file not found: {font_path_str} (resolved to: {font_file_path_obj.resolve() if not font_file_path_obj.is_absolute() else font_file_path_obj}) for family {font_conf['font_family']}")
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
        else:
            render_data['font_configs'] = []

        # --- Format date if date_output_format is provided and date is a datetime.date object ---
        if 'date_output_format' in render_data and render_data['date_output_format'] and 'date' in render_data and render_data['date'] is not None:
            if isinstance(render_data['date'], datetime.date):
                try:
                    formatted_date = render_data['date'].strftime(render_data['date_output_format'])
                    render_data['date'] = formatted_date
                except ValueError as ve:
                    print(f"Warning: Invalid date_output_format string '{render_data['date_output_format']}'. Error: {ve}")
                except Exception as e:
                    print(f"Warning: Could not format date using format '{render_data['date_output_format']}'. Error: {e}")
            else: # Date is not a date object (e.g. it's a string)
                print(f"Warning: 'date_output_format' was provided, but 'date' field is a string ('{render_data['date']}'). "
                      "Formatting will be skipped. Provide 'date' as a datetime.date object for formatting.")

        # --- Setup Jinja and render ---
        base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        template_dir = os.path.join(base_dir, 'templates')

        env = Environment(loader=FileSystemLoader(template_dir), autoescape=True)
        template = env.get_template('check.html')
        html_string = template.render(render_data)

        html_obj = HTML(string=html_string, base_url=template_dir)

        if output_path:
            html_obj.write_pdf(output_path)
            print(f"Check PDF generated and saved to {output_path}")
            return True
        else:
            pdf_bytes = html_obj.write_pdf()
            print("Check PDF generated as bytes.")
            return pdf_bytes

    except Exception as e:
        print(f"Error generating check: {e}")
        return None

if __name__ == '__main__':
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    # Test date formatting
    sample_data_date_format = {
        'bank_name': 'Date Format Bank',
        'check_number': '4001',
        'payee_name': 'Formatted Outputs Inc.',
        'amount_numeric': '100.00',
        'amount_words': 'ONE HUNDRED AND 00/100',
        'memo': 'Date format test',
        'routing_number': '010101010',
        'account_number': '999888777',
        'bank_logo_url': None,
        'font_configs': [],
        'date': datetime.date(2023, 11, 25),
        'date_output_format': '%d-%b-%Y'
    }
    output_file_date_fmt = os.path.join(project_root, 'generated_check_date_format_test.pdf')
    print(f"\nTesting date formatting with datetime.date object (format: {sample_data_date_format['date_output_format']})...")
    if generate_check(sample_data_date_format, output_path=output_file_date_fmt):
        print(f"Test check with date formatting saved to {output_file_date_fmt}")
    else:
        print(f"Failed to save test check with date formatting to {output_file_date_fmt}")

    # Test with date as string (formatting should be skipped)
    sample_data_date_string = sample_data_date_format.copy() # Re-use most data
    sample_data_date_string['date'] = "November 25, 2023" # Input as string
    # date_output_format is still '%d-%b-%Y', but it should be ignored with a warning
    output_file_date_str = os.path.join(project_root, 'generated_check_date_string_test.pdf')
    print(f"\nTesting date formatting with date as string (format: {sample_data_date_string['date_output_format']})...")
    if generate_check(sample_data_date_string, output_path=output_file_date_str):
        print(f"Test check with date as string (formatting skipped) saved to {output_file_date_str}")
    else:
        print(f"Failed to save test check with date as string to {output_file_date_str}")

    # Test with invalid date_output_format string
    sample_data_invalid_format = sample_data_date_format.copy() # Re-use most data
    sample_data_invalid_format['date'] = datetime.date(2023, 11, 26) # Use datetime.date object
    sample_data_invalid_format['date_output_format'] = "%Z%Z%Z" # Invalid format string
    output_file_invalid_fmt = os.path.join(project_root, 'generated_check_invalid_date_format_test.pdf')
    print(f"\nTesting date formatting with invalid format string ({sample_data_invalid_format['date_output_format']})...")
    if generate_check(sample_data_invalid_format, output_path=output_file_invalid_fmt):
        print(f"Test check with invalid date format saved to {output_file_invalid_fmt}")
    else:
        print(f"Failed to save test check with invalid date format to {output_file_invalid_fmt}")


    # Clean up generated test files
    files_to_cleanup = [output_file_date_fmt, output_file_date_str, output_file_invalid_fmt]
    # Add any dummy files if they were created in this specific __main__ block
    # For now, assuming previous dummy files (logo, font) are handled by their respective test runs
    # or a more general cleanup script if this __main__ was part of a larger test execution.
    # We will clean up files created by THIS __main__ block.

    print("\nCleaning up generated test PDFs from this run...")
    for f_path in files_to_cleanup:
        if os.path.exists(f_path):
            try:
                os.remove(f_path)
                print(f"Removed: {f_path}")
            except OSError:
                print(f"Could not remove test file: {f_path}")

    # General cleanup of known dummy files that might be left from other __main__ test runs
    # if this script is run standalone multiple times.
    # This is more for convenience during development.
    print("\nAttempting cleanup of common dummy files...")
    for dummy_file_name in ["dummy_logo.png", "dummy_logo_for_font_test.png",
                            "dummy_font.ttf", "dummy_font_for_check_test.ttf",
                            "generated_check_with_logo_test.pdf",
                            "generated_check_with_font_test.pdf"]:
        dummy_path = os.path.join(project_root, dummy_file_name)
        if os.path.exists(dummy_path):
            try:
                os.remove(dummy_path)
                print(f"Cleaned up common dummy/test file: {dummy_path}")
            except OSError:
                print(f"Could not cleanup common dummy/test file: {dummy_path}")
                pass
