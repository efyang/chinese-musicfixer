import argparse
import mutagen.easyid3
parser = argparse.ArgumentParser(description='Get/Set m4a metadata')
parser.add_argument("filepath")
parser.add_argument("tag")
parser.add_argument('--set-value', action = "store", dest = "tag_value")

args = parser.parse_args()
f = mutagen.easyid3.EasyID3(args.filepath)

try:
    print(f[args.tag][0])
    if args.tag_value:
        f[args.tag] = args.tag_value
        f.save()
except:
    print("")
