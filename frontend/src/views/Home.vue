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
    <train-arrival-editor v-model="currentStation" :initialMessageList="currentMessageList" :open="showArrivalWidget" @close="closeTrainArrival(false, null)" @save="(val: MessageItem[]) => closeTrainArrival(true, val)" />
    <alert-editor v-model="currentAlertList" :open="showAlertWidget" @close="closeAlerts(false)" @update:modelValue="closeAlerts(true)" />
  </base-layout>
</template>

<script lang="ts" setup>
import BaseLayout from '@/components/BaseLayout.vue'
import { onMounted, ref, reactive } from 'vue'
import { useDatabaseList  } from 'vuefire'
import { ref as dbRef, getDatabase, push, set } from 'firebase/database'
import TrainArrivalEditor from '@/components/TrainArrivalEditor.vue'
import AlertEditor from '@/components/AlertEditor.vue'
import { GenericWidget, MessageItem } from '@/models'

const widgetDb = dbRef(getDatabase(), 'widgets')
const activeWidgets = useDatabaseList<GenericWidget>(widgetDb)
let currentItemId: string | null = null;

// push(widgetDb, defaultTrain)

const headers = reactive([
  {
    title: 'Widget Name',
    key: 'name',
  },
  { title: 'Configure', key: 'configure', sortable: false },
])

const showArrivalWidget = ref(false);
const showAlertWidget = ref(false);
const currentStation = ref(0);
const currentMessageList = ref([]);
const currentAlertList = ref([]);

const showDialog = (item: any) => {
  if (item.value === 'DCMetroTrainArrivalWidget') {
    // console.log(item.raw)
    currentItemId = item.raw.id;
    currentStation.value = item.raw.station_id;
    currentMessageList.value = item.raw.messages || [];
    showArrivalWidget.value = true;
  }
  else if (item.value === 'DCMetroAlertsWidget') {
    // console.log(item.raw)
    currentItemId = item.raw.id;
    currentAlertList.value = item.raw.alerts || [];
    showAlertWidget.value = true;
  }
}

const closeTrainArrival = (save: boolean, messages: MessageItem[] | null) => {
  if (save) {
    console.log(currentStation.value);
    const newObj: any = {
      "name": "DCMetroTrainArrivalWidget",
      "station_id": currentStation.value,
      "messages": []
    }
    if (messages !== null && messages.length > 0) {
      newObj.messages = messages.map(msg => ({...msg, time: msg.time.getTime()}))
    }
    set(dbRef(getDatabase(), 'widgets/' + currentItemId), newObj);
  }
  showArrivalWidget.value = false;
}

const closeAlerts = (save: boolean) => {
  if (save) {
    const newObj: any = {
      "name": "DCMetroAlertsWidget",
      "alerts": currentAlertList.value
    }
    set(dbRef(getDatabase(), 'widgets/' + currentItemId), newObj);
  }
  showAlertWidget.value = false;
}

// onMounted(() => {
// })

</script>

<style>
  .widgetTable .v-card-text {
    padding: 0;
  }
</style>