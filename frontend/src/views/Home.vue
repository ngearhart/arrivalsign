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
    <train-arrival-editor :open="showArrivalWidget" @close="showArrivalWidget = false" />
  </base-layout>
</template>

<script lang="ts" setup>
import BaseLayout from '@/components/BaseLayout.vue'
import { onMounted, ref, reactive } from 'vue'
import { useDatabaseList  } from 'vuefire'
import { ref as dbRef, getDatabase, push } from 'firebase/database'
import TrainArrivalEditor from '@/components/TrainArrivalEditor.vue'

const widgetDb = dbRef(getDatabase(), 'widgets')
const activeWidgets = useDatabaseList<GenericWidget>(widgetDb)

// push(widgetDb, defaultTrain)

const headers = reactive([
  {
    title: 'Widget Name',
    key: 'name',
  },
  { title: 'Configure', key: 'configure', sortable: false },
])

const showArrivalWidget = ref(false);

const showDialog = (item: any) => {
  if (item.value === 'DCMetroTrainArrivalWidget') {
    showArrivalWidget.value = true;
  }
}

// onMounted(() => {
// })

</script>

<style>
  .widgetTable .v-card-text {
    padding: 0;
  }
</style>