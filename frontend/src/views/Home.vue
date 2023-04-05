<template>
  <base-layout>
    <v-card title="Widgets">
      <template v-slot:text>
        <v-data-table
          :headers="headers"
          :items="activeWidgets"
          item-value="name"
          class="elevation-1"
        ></v-data-table>
      </template>
    </v-card>
  </base-layout>
</template>

<script lang="ts" setup>
import BaseLayout from '@/components/BaseLayout.vue'
import { onMounted, ref, reactive } from 'vue'
import { useDatabaseList  } from 'vuefire'
import { ref as dbRef, getDatabase } from 'firebase/database'

const activeWidgets = useDatabaseList<GenericWidget>(dbRef(getDatabase(), 'widgets'))

const defaultTrain: DCMetroTrainArrivalWidget = {
  custom_trains: [],
  name: "DCMetroTrainArrivalWidget",
  station_id: "K07",
  enabled: true
}


// activeWidgets.value.push(defaultTrain)

const headers = reactive([
  {
    title: 'Widget Name',
    key: 'name',
  },
  { title: 'Configure', sortable: false },
])

onMounted(() => {
})

</script>
