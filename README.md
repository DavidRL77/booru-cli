# booru-cli
A command line interface to fetch posts from booru sites and operate on their data in various ways.

## Supported sites
| Site | Auth Required |
|------|---------------|
| [Safebooru](https://safebooru.org) (default)| **No** |
| [Gelbooru](https://gelbooru.com) | **Yes** |
| [Rule34](https://rule34.xxx) | **Yes** |

More clients may be implemented in the future.

## Features
- Displaying post's data in JSON format.
- Displaying a post's file url.
- Downloading a post to a custom directory (or to the default temp directory) and displaying its downloaded location.

These features are made with scripting and automation in mind so they can be passed or piped into other programs. For example, one could query a post's height or width using [jq](https://jqlang.org/), or open an image url directly in-terminal using [timg](https://github.com/hzeller/timg).
> **Note:** Gelbooru urls can't be displayed directly and have to be downloaded first.

## Installation
No official way to install just yet, but you can build it from source in the meantime.

## Examples
Print the url of a single, random image that:
- Includes the tag: `smile`
- Has a rating of: `safe`
```
booru-cli url -r safe -t smile -s random -l 1
``` 

Download a single image into this app's temp folder that:
- Includes the tags: `frieren` and `real_life`
- Excludes the tags: `cosplay`
- Has the highest `score`
```
booru-cli download -t frieren -t real_life -T cosplay -s score -l 1
```

Opening a fetched result using another program:
```
timg $(booru-cli url -t pink_hair)
```