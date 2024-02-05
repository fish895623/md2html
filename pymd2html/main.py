from flask import Flask

from pymd2html import __version__

app = Flask("pymd2html")


def main():
    app.run()
