from unittest.mock import patch
from unittest import TestCase
from types import SimpleNamespace
from datetime import datetime, timedelta

from widgets.arrival import ArrivalWidget


def fake_get_firebase_messages(desired_output):
    def fb_return(url, _):
        return {
            'test_id': {
                'name': ArrivalWidget.WIDGET_NAME,
                'messages': desired_output
            }
        }
    return fb_return


class ArrivalWidgetTests(TestCase):

    @patch('utils.MetroApi.fetch_train_predictions')
    @patch('widgets.arrival.get_firebase')
    def test_get_lines_to_display_no_custom(self, get_fb_mock, fetch_train_predictions_mock):
        get_fb_mock.return_value = SimpleNamespace()
        fetch_train_predictions_mock.return_value = []

        widget = ArrivalWidget()

        get_fb_mock.return_value.get = fake_get_firebase_messages([])
        self.assertEqual(widget.get_lines_to_display(), [])

    @patch('utils.MetroApi.fetch_train_predictions')
    @patch('widgets.arrival.get_firebase')
    def test_get_lines_to_display_custom_sticky_only(self, get_fb_mock, fetch_train_predictions_mock):
        get_fb_mock.return_value = SimpleNamespace()
        fetch_train_predictions_mock.return_value = []

        widget = ArrivalWidget()

        get_fb_mock.return_value.get = fake_get_firebase_messages([
            {
                'message': 'one',
                'sticky': 'true',
                'time': datetime.now().timestamp() * 1000
            }
        ])
        self.assertEqual(widget.get_lines_to_display(), [])
