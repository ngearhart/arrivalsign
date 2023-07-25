<template>
  <v-container class="fill-height" fluid>
    <v-row align='center' align-content='center' justify='center'>
      <v-col align-self='center' style='text-align: center'>
        <p class="mb-2">You have been logged out.</p>
        <v-btn variant="tonal" href="/login">Click here to log back in</v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { getCurrentUser } from "vuefire"; 
import router from '../router';
import { onMounted } from 'vue'

onMounted(async () => {
  const currentUser = await getCurrentUser()
  if (currentUser) {
    router.push('/')  // Redirect back home if the user actually is logged in
  }
})

</script>
