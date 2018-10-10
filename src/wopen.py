#!/usr/bin/env python3
import os, sys, logging
import subprocess, time
import argparse, json
import webbrowser
import nbformat
from nbconvert import HTMLExporter

h_jupyter = None

def open_notebook(pawprint):
    COLLECTION = lambda pawprint, path: os.path.join(argv.collection, pawprint, path)
    JUPYTER_NOTEBOOK = lambda pawprint, path: os.path.join('http://localhost:8888/notebooks', pawprint, path).replace('\\', '/')
    fname = COLLECTION(pawprint, argv.notebook)
    if os.path.exists(fname):
        logging.info('nbmerge: Opening: {}'.format(fname))
        #global h_jupyter
        #if not h_jupyter:
        #    try:
        #        h_jupyter = subprocess.Popen(r'jupyter notebook --no-browser --NotebookApp.token="" --NotebookApp.passwork=""', cwd = argv.collection, shell = True)
        #        time.sleep(3)
        #    except Exception as e:
        #        logging.error('nbmerge: subprocess.Popen(): {}'.format(e))
        webbrowser.open(JUPYTER_NOTEBOOK(pawprint, argv.notebook))
        #try:
        #    with open(fname) as f_in:
        #        notebook = nbformat.reads(f_in.read())
        #    html_exporter = HTMLExporter()
        #    (body, resources) = html_exporter.from_notebook_node(notebook)
        #    with open(r'E:\sample.html', 'wb') as f_out:
        #        f_out.write(body)
        #except (Exception, e):
        #    print(e)
    else:
        logging.info('nbmerge: Notebook does not exist: {}'.format(fname))

def main():
    args = argparse.ArgumentParser()
    args.add_argument('collection', help='path to collection directory')
    args.add_argument('notebook', help='path to the notebook')
    args.add_argument('pawprints')
    global argv
    argv = args.parse_args()
    with open(argv.pawprints) as f:
        for line in f:
            pawprint = line.strip()
            open_notebook(pawprint)
            

if __name__ == '__main__':
    main()
