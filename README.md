# Quickiller
There are always programs such as chrome that keep eating up your resources even when closed! The only way to prevent this is to kill all the chrome process manually which can be tedious at times. But quickiller is an easy-to-use CLI tool that automatically kills all the processes for you in a short amount of time!

## Release 1.1
- Added universal config file
- Fixed compiler warnings

### Planned features
- Profiles for different kill lists
- GUI, but not sure yet.
- Easier setup process
## Features
- Kill all processes in customizable list (duh)
- Add process to kill
- Remove process to kill
- List process that would be killed when quickiller run command is ran
- List all system processes (To help when adding a process to the list)

## Bugs
Error handling will be taken more seriously in later versions, but for now there will be errors. If you encounter an error, please report it in the issues section.

## Set up (Windows 10)
Setting up quickiller will take couple minutes but it wont be like this in the near future.
1. Download the EXE file
2. Make a directory/folder anywhere, perferably in C:\Users\YOUR NAME so it should look like C:\Users\YOUR NAME\quickiller
3. Drag the EXE file into C:\Users\YOUR NAME\quickiller
4. type "environment variables" into the search bar and click "Edit the system environment variables"
5. At the bottom, click "Environment variables..."
6. There should be 2 boxes, search for Path in System variables
![tutorial image 1](https://i.imgur.com/XSJnawx.png)
7. Double click on Path 
8. Click "New" on the right side
9. Paste in the directory to quickiller (Example: C:\Users\YOUR NAME\quickiller) 
NOTE: Dont include the exe file in the directory!
10. Click OK on all the windows and try entering "quickiller help" on the command prompt! It will set up the first time and then try it again.