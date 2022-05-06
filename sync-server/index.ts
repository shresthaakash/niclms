import express from "express";
import PouchDB from 'pouchdb';
PouchDB.plugin(require('pouchdb-adapter-memory'));

const app = express();

app.use('/db', require('express-pouchdb')(PouchDB.defaults({adapter:'memory'})));

app.listen(3000);