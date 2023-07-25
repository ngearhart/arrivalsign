<template>
  <base-layout>
    <v-card title="Widgets" class="widgetTable">
      <template v-slot:text>
        <v-data-table
          :headers="headers"
          :items="activeWidgets"
          item-value="name"
          class="elevation-1"
        >
          <template v-slot:item.configure="{ item }">
            <v-btn color="primary" icon="mdi-cog" @click="showDialog(item)"></v-btn>
          </template>
        </v-data-table>
      </template>
    </v-card>
    <train-arrival-editor v-model="currentStation" :open="showArrivalWidget" @close="closeTrainArrival(false)" @save="closeTrainArrival(true)" />
  </base-layout>
</template>

<script lang="ts" setup>
import BaseLayout from '@/components/BaseLayout.vue'
import { onMounted, ref, reactive } from 'vue'
import { useDatabaseList  } from 'vuefire'
import { ref as dbRef, getDatabase, push, set } from 'firebase/database'
import TrainArrivalEditor from '@/components/TrainArrivalEditor.vue'
import { GenericWidget } from '@/models'

const widgetDb = dbRef(getDatabase(), 'widgets')
const activeWidgets = useDatabaseList<GenericWidget>(widgetDb)
let currentItemId: string | null = null;

// push(widgetDb, defaultTrain)

const headers = reactive([
  {
    title: 'Widget Name',
    key: 'name',
  },
  {
    title: 'Id',
    key: 'id',
  },
  {
    title: 'Station',
    key: 'station_id',
    hidden: true,
  },
  { title: 'Configure', key: 'configure', sortable: false },
])

const showArrivalWidget = ref(false);
const currentStation = ref(0);

const showDialog = (item: any) => {
  if (item.value === 'DCMetroTrainArrivalWidget') {
    currentItemId = item.columns.id;
    currentStation.value = item.columns.station_id;
    showArrivalWidget.value = true;
  }
}

const closeTrainArrival = (save: boolean) => {
  if (save) {
    console.log(currentStation.value);
    set(dbRef(getDatabase(), 'widgets/' + currentItemId), { "name": "DCMetroTrainArrivalWidget", "station_id": currentStation.value });
  }
  showArrivalWidget.value = false;
}

// onMounted(() => {
// })

</script>

<style>
  .widgetTable .v-card-text {
    padding: 0;
  }
</style>