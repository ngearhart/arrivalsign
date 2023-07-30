<template>
  <v-dialog :model-value="open" width="700" persistent>
    <v-card>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="12" md="2" style="display: flex; justify-content: center; align-items: center  ;">
              Station:
            </v-col>
            <v-col cols="12" md="10">
              <v-select-2
                :options="store.STATIONS"
                v-model="value"
                label="name"
              ></v-select-2>
            </v-col>
          </v-row>
          <v-row>
            <v-col>
              <v-data-table
                :headers="headers"
                :items="customMessages"
                :sort-by="[{ key: 'time', order: 'asc' }]"
                class="elevation-1"
              >
                <template v-slot:top>
                  <v-toolbar
                    flat
                  >
                    <v-toolbar-title>Custom Messages</v-toolbar-title>
                    <v-divider
                      class="mx-4"
                      inset
                      vertical
                    ></v-divider>
                    <v-spacer></v-spacer>
                    <v-dialog
                      v-model="dialog"
                      max-width="500px"
                    >
                      <template v-slot:activator="{ props }">
                        <v-btn
                          color="primary"
                          dark
                          class="mb-2"
                          v-bind="props"
                        >
                          Add Message
                        </v-btn>
                      </template>
                      <v-card>
                        <v-card-title>
                          <span class="text-h5">{{ formTitle }}</span>
                        </v-card-title>

                        <v-card-text>
                          <v-container>
                            <v-row>
                              <v-col
                                cols="12"
                                sm="6"
                                md="4"
                              >
                                <v-text-field
                                  v-model="editedItem.message"
                                  label="Message"
                                ></v-text-field>
                              </v-col>
                              <v-col
                                cols="12"
                                sm="6"
                                md="4"
                              >
                                <v-text-field
                                  v-model="editedItem.time"
                                  label="Time"
                                ></v-text-field>
                              </v-col>
                              <v-col
                                cols="12"
                                sm="6"
                                md="4"
                              >
                                <v-text-field
                                  v-model="editedItem.sticky"
                                  label="Sticky"
                                ></v-text-field>
                              </v-col>
                            </v-row>
                          </v-container>
                        </v-card-text>

                        <v-card-actions>
                          <v-spacer></v-spacer>
                          <v-btn
                            color="blue-darken-1"
                            variant="text"
                            @click="close"
                          >
                            Cancel
                          </v-btn>
                          <v-btn
                            color="blue-darken-1"
                            variant="text"
                            @click="save"
                          >
                            Save
                          </v-btn>
                        </v-card-actions>
                      </v-card>
                    </v-dialog>
                    <v-dialog v-model="dialogDelete" max-width="500px">
                      <v-card>
                        <v-card-title class="text-h5">Are you sure you want to delete this item?</v-card-title>
                        <v-card-actions>
                          <v-spacer></v-spacer>
                          <v-btn color="blue-darken-1" variant="text" @click="closeDelete">Cancel</v-btn>
                          <v-btn color="blue-darken-1" variant="text" @click="deleteItemConfirm">OK</v-btn>
                          <v-spacer></v-spacer>
                        </v-card-actions>
                      </v-card>
                    </v-dialog>
                  </v-toolbar>
                </template>
                <template v-slot:item.actions="{ item }">
                  <v-icon
                    size="small"
                    class="me-2"
                    @click="editItem(item.raw)"
                  >
                    mdi-pencil
                  </v-icon>
                  <v-icon
                    size="small"
                    @click="deleteItem(item.raw)"
                  >
                    mdi-delete
                  </v-icon>
                </template>
                <template v-slot:no-data>
                  <v-btn
                    color="primary"
                    @click="initialize"
                  >
                    Reset
                  </v-btn>
                </template>
              </v-data-table>
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

import { computed, reactive, ref, watch, nextTick } from 'vue'
import { useAppStore } from '@/store/app'

interface MessageItem {
  message: string,
  time: number,
  sticky: boolean
}

const store = useAppStore()
const props = defineProps(['open', 'modelValue'])
const emit = defineEmits(['save', 'update:modelValue'])

const headers = reactive([
  {
    title: 'Message',
    key: 'message',
  },
  {
    title: 'Target Time',
    key: 'time',
  },
  {
    title: 'Sticky',
    key: 'sticky',
  },
  { title: 'Actions', key: 'actions', sortable: false },
])
const dialog = ref(false)
const dialogDelete = ref(false)
const customMessages = reactive<MessageItem[]>([])
const editedIndex = ref(-1)
const editedItem = ref<MessageItem>({
  message: '',
  time: 0,
  sticky: false
})
const defaultItem = ref<MessageItem>({
  message: '',
  time: 0,
  sticky: false
})

const formTitle = computed({
  get() {
    return editedIndex.value === -1 ? 'New Message' : 'Edit Message'
  },
  set(value) {}  // don't set this
})

const value = computed({
  get() {
    return store.STATIONS.find(station => station.code === props.modelValue)
  },
  set(value) {
    emit('update:modelValue', value?.code)
  }
})

watch(dialog, (val) => val || close())
watch(dialogDelete, (val) => val || closeDelete())

const editItem = (item: MessageItem) => {
  editedIndex.value = customMessages.indexOf(item)
  editedItem.value = Object.assign({}, item)
  dialog.value = true
}

const deleteItem = (item: MessageItem) => {
  editedIndex.value = customMessages.indexOf(item)
  editedItem.value = Object.assign({}, item)
  dialogDelete.value = true
}

const deleteItemConfirm = () => {
  customMessages.splice(editedIndex.value, 1)
  closeDelete()
}

const close = () => {
  dialog.value = false
  nextTick(() => {
    editedItem.value = Object.assign({}, defaultItem.value)
    editedIndex.value = -1
  })
}

const closeDelete = () => {
  dialogDelete.value = false
  nextTick(() => {
    editedItem.value = Object.assign({}, defaultItem.value)
    editedIndex.value = -1
  })
}

const save = () => {
  if (editedIndex.value > -1) {
    Object.assign(customMessages[editedIndex.value], editedItem.value)
  } else {
    customMessages.push(editedItem.value)
  }
  close()
}


</script>
  