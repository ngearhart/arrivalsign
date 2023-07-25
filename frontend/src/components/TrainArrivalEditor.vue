<template>
  <v-dialog :model-value="open" width="512" persistent>
    <v-card>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="12" md="12">
              <v-select-2
                :options="store.STATIONS"
                v-model="value"
                label="name"
              ></v-select-2>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>
      <v-card-actions>
        <v-container>
          <v-row>
            <v-col cols="12" md="12" style="text-align: center;">
              <v-btn color="primary" @click="$emit('save')">Save</v-btn>
              <v-btn color="gray" @click="$emit('close')">Cancel</v-btn>
            </v-col>
          </v-row>
        </v-container>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
  
<script lang="ts" setup>

import { computed } from 'vue'
import { useAppStore } from '@/store/app'

const store = useAppStore()
const props = defineProps(['open', 'modelValue'])
const emit = defineEmits(['save', 'update:modelValue'])

const value = computed({
  get() {
    return store.STATIONS.find(station => station.code === props.modelValue)
  },
  set(value) {
    emit('update:modelValue', value?.code)
  }
})


</script>
  