# Rust Archiver by Args

Created by Blake@<129040985+MrDwarf7@users.noreply.github.com> 2024/01/17

## Readme document/Quick Ref

### Description

### Usage

Program is written in Rust, and takes from 1 to 3 input arguments via the command line it is to be called from a Command Line Interface (CLI).

Program is designed to assist wtih archiving large volumes of files within a given source folder
This can be useful for input folders of files, or reporting folder locations.

(The types of document should not be relevant to the program as it handles via byte stream,
but was deisgned for various Excel compatible formats to archive report/input files)

In practice the program takes in several arguments, and will archive all files within the source folder given in the first argument, and move them to a subfolder of the source folder, with the name given in the second argument.

- [Mandatory] Given a folder as the first input argument,

- [Optional] a folder name (EG: < archive > without quotations of the angled brackets), and a date.

- [Optional] If a date is not supplied, it will default to 2023/01/01.

Base example of params.

```shell
./archiver.exe <FOLDER_WITH_FILES> <ARCHIVE_FOLDER_NAME> <DATE>
```

If you gave for example:

```shell
./archiver.exe C:\Users\user\Documents\input_folder my_archive 2020/01/01
```

You would have a resulting file structure that would look something like this:

(Assuming dates are handles at your discretion)

```shell
|   | C:\Users\user\Documents\input_folder
|   ----| C:\Users\user\Documents\input_folder\my_archive
|   ----| file1Original.xlsx
|   ----| file2Original.xlsx
|   ----| file3Original.xlsx
|   ----| file4Original.xlsx
|   --------| C:\Users\user\Documents\input_folder\my_archive\my_archive.zip
|   ------------| file1Original.xlsx
|   ------------| file2Original.xlsx
|   ------------| file3Original.xlsx
|   ------------| file4Original.xlsx
```

#### Input

### [Mandatory]: First argument -

The program will take the first argument as the source folder to archive from.

Example:

```shell
./archiver.exe C:\Users\user\Documents\input_folder
```

___

### [Optional]: Second argument -

The program will take the second argument as the destination folder to archive to.

Default: "archive"

This is **assumed** the folder is to be created within the source folder given in argument 1.
archive_to_folder is treated as a subfolder of the source folder, and will be created if it does not exist.
With the default name being "archive".

Note: Do not surround the name of the foldr on the CLI with quotes if not required, program was not built nor tested with quotations or escape characters.

Example:

```shell
./archiver.exe C:\Users\user\Documents\input_folder my_archive
```

___

### [Optional]: Third argument -

Third argument given is a human readable date format, in the format of YYYY/MM/DD.

Default: 2023/01/01

This is used as a marker reference, all files that are BEFORE the given date are moved, and copied into the zip file. Example being, if you gave the date 2020/01/01, all files that were created before 2020/01/01 would be moved to the zip file.

Example:

```shell
./archiver.exe C:\Users\user\Documents\input_folder my_archive 2020/01/01
```

___

### Output

The program will output a zip file, with the name of the folder given in the second argument, and the date given in the third argument.
archiver will also ask a couple of questions to after the process has finished.
Asking the user if they would like to dump a copy of the files that were moved to a CSV file.

If the user choose (Y/y) yes, then they will also be prompted for a directory to save the CSV file to, if no directory is given, the CSV file will be saved to the same directory as the program was called from.
