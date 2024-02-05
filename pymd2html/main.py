from flask import Flask

from pymd2html import __version__

app = Flask("pymd2html")


@app.route("/version")
def get_version():
    return __version__


def main():
    app.run()


if __name__ == "__main__":
    main()
