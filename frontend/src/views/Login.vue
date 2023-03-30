<template>
  <div>
    Login
  </div>
</template>

<script lang="ts" setup>
import { getCurrentUser } from "vuefire"; 
import router from '../router';
import { useRoute } from 'vue-router'
import { getAuth, signInWithRedirect } from 'firebase/auth'
import firebase from "firebase/compat/app"
import { onMounted } from 'vue'
import "firebase/compat/auth"
import "firebase/compat/firestore"

// within the Page component displayed for the `/login` route
onMounted(async () => {
  const route = useRoute()
  const currentUser = await getCurrentUser()
  if (currentUser) {
    const to =
        route.query.redirect && typeof route.query.redirect === 'string'
        ? route.query.redirect
        : '/'

    router.push(to)
  } else {
    const provider = new firebase.auth.GoogleAuthProvider();
    signInWithRedirect(getAuth(), provider)
    // .then((result: any) => {
    //   const token = result.credential.accessToken;
    //   const user = result.user;
    // }).catch(console.log)
  }
})

</script>
