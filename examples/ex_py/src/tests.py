import unittest
from main import hello_name

class TestMain(unittest.TestCase):

    def test_hello_name(self):
        self.assertEqual(hello_name("Ariful"), 'Hello World and hello Ariful')

if __name__ == '__main__':
    unittest.main()