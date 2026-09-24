# booru-cli
A command line interface to fetch posts from booru sites and operate on their data in various ways.

## Supported sites
| Site | Auth Required |
|------|---------------|
| [Safebooru](https://safebooru.org) (default)| **No** |
| [Gelbooru](https://gelbooru.com) | **Yes** |
| [Rule34](https://rule34.xxx) | **Yes** |
| [Konachan](https://konachan.com/) | **No** |

More clients may be implemented in the future.

## Features
- Displaying post's data in JSON format.
- Displaying a post's file url.
- Downloading a post to a custom directory (or to the default temp directory) and displaying its downloaded location.

These features are made with scripting and automation in mind so they can be passed or piped into other programs. For example, one could query a post's height or width using [jq](https://jqlang.org/), or open an image url directly in-terminal using [timg](https://github.com/hzeller/timg).
> **Note:** Gelbooru urls can't be displayed directly and have to be downloaded first.

## Installation
Go to the [latest release](https://github.com/DavidRL77/booru-cli/releases/latest) and download the correct archive for your operating system.

Extract the archive and move the binary to a suitable location so it can be used from the command line.

<details>
<summary>Linux</summary>

A common location for custom binaries in Linux is: `/usr/local/bin`

</details>

<details>
<summary>Windows</summary>

Place the binary in your folder of choice and [add the folder to your PATH](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/)

</details>

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
booru-cli url -t pink_hair --open=timg
```
