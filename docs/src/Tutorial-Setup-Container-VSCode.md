# VS Code Development Container Setup

This container is designed to integrate natively into VS Code using the Remote Containers extension.  VS Code an IDE framework that is usable run on Mac, Windows, or Linux, and can be downloaded [here](https://code.visualstudio.com/download).

> **Note**
> *MacOS Users*:  VS Code is usable on both x64 and M1 Macs.  One way to install it for MacOS is to use the package manager *brew*.  To get *brew*, please follow the instuctions [here](https://brew.sh/).  When you have *brew*, run `brew install --cask visual-studio-code` to obtain VS Code.

1. Ensure that VS Code is installed, and the *wasm-toolkit* repository has been cloned.

1. In VS Code, install the *Remote - Containers VS Code Extension* if you do not already have it.

1. In VS Code, type F1 and search for “Open Folder in Container”.

1. Navigate to the directory where you cloned the *wasm-toolkit* repo, and click Open.

1. VS Code will now build the container, which may take quite a while.

When the container build has completed, you should see *wasm-toolkit* repository's file tree appear in the *EXPLORER* panel on the left side of the VS Code window.  For the purposes of this tutorial, it will be most convenient if you store your code in a directory relative to the repository.  Here's one way to do this:

1. Right-click on the empty space in the EXPLORER panel, and select `New Folder`.

1. Type `wasm-tutorial` and press Enter.  There should now be a sub-folder by this name in the tree.  Put any files you create in here.

> **Note**
> As you work through this tutorial, please be sure to execute your commands *inside the VS Code terminal window* so that you have access to the necessary tools.

Next, let's pick an [example](Tutorial-Examples.md) to work through.  

