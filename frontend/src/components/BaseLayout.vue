<template>
  <v-app>
    <v-app-bar>
      <v-toolbar-title>Arrival Sign Control Panel</v-toolbar-title>
      <div>
        Logged in as {{ userData.name }}
      </div>
      <v-btn variant="tonal" color="primary" class="ml-2 mr-2" @click="logout">Log out</v-btn>
    </v-app-bar>

    <v-main class="bg-grey-lighten-2">
      <v-container>
        <slot></slot>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup>
import { getCurrentUser } from "vuefire";
import { onMounted, reactive } from "vue";
import { getAuth, signOut } from 'firebase/auth'
import { useRouter } from 'vue-router'

const userData = reactive({
  name: "",
});
const router = useRouter();

onMounted(async () => {
  const currentUser = await getCurrentUser();
  userData.name = currentUser.displayName;
});

const logout = () => {
  signOut(getAuth()).then(() => {
    router.push('/logged-out')
  })
}
</script>
