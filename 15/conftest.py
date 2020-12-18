import pytest

# skip long tests (from 2nd part) unless specifically requested

def pytest_addoption(parser):
    parser.addoption(
        '--regress', action='store_true', default=False,
        help='run all tests (slow)'
    )


def pytest_collection_modifyitems(config, items):
    if not config.getoption('--regress'):
        skip = pytest.mark.skip(reason='slow regression test')
        for item in items:
            if '::test2' in item.nodeid:
                item.add_marker(skip)
