import express from "express";
import PouchDB from 'pouchdb';
PouchDB.plugin(require('pouchdb-adapter-memory'));

const app = express();

app.use('/', require('express-pouchdb')(PouchDB.defaults({adapter:'memory'}),{
    mode: 'fullCouchDB',
    overrideMode: {
      include: ['routes/fauxton']
    }
  }));

app.listen(3000);