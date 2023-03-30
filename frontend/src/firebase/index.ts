import { initializeApp } from 'firebase/app'

export const firebaseApp = initializeApp({
    apiKey: "AIzaSyCyT8dhe_8Y4OiJ7QuYc9aknq3jF2flDKU",
    authDomain: "arrivalsign.firebaseapp.com",
    databaseURL: "https://arrivalsign-default-rtdb.firebaseio.com",
    projectId: "arrivalsign",
    storageBucket: "arrivalsign.appspot.com",
    messagingSenderId: "421217409615",
    appId: "1:421217409615:web:c055d39423b90e7312283a",
    measurementId: "G-N80HNY5XE9"
})

// used for the firestore refs
// const db = getFirestore(firebaseApp)

// here we can export reusable database references
// export const todosRef = collection(db, 'todos')
