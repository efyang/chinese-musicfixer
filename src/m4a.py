import argparse
import mutagen.mp4
parser = argparse.ArgumentParser(description='Get/Set m4a metadata')
parser.add_argument("filepath")
parser.add_argument("tag")
parser.add_argument('--set-value', action = "store", dest = "tag_value")

args = parser.parse_args()
f = mutagen.mp4.MP4(args.filepath)

def getkey(x):
    return {
        'title': '\xa9nam',
        'album': '\xa9alb',
        'artist': '\xa9ART',
        'album_artist': 'aART'
    }[x]

if args.tag_value:
    f[getkey(args.tag)] = args.tag_value
    f.pprint()
    f.save()
else:
    print(f[getkey(args.tag)][0])
