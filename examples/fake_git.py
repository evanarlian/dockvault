from argparse import ArgumentParser

if __name__ == "__main__":
    parser = ArgumentParser()
    subparsers = parser.add_subparsers(required=True)
    clone = subparsers.add_parser("clone")
    clone.add_argument("repo")
    commit = subparsers.add_parser("commit")
    commit.add_argument("-m", type=str, required=True)
    parser.parse_args()
