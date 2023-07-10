<template>
  <v-dialog :model-value="open" width="512">
    <v-card>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="12" md="12">
              <v-autocomplete
                label="Station"
                :items="store.STATIONS"
                item-title="name"
                item-value="value"
              ></v-autocomplete>
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

import { useAppStore } from '@/store/app'

const store = useAppStore()
const props = defineProps(['open', 'station_id'])

</script>
  