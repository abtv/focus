<script setup lang="ts">
  import { ref, onMounted, nextTick } from "vue"
  import { invoke } from "@tauri-apps/api/tauri"

  const emit = defineEmits(["create", "open", "error"])

  interface SearchResult {
    action: string;
    value: string;
    label: string;
  }

  const value = ref("")
  const items = ref<SearchResult[]>([])
  const input = ref<any>(null)

  const onSearch = async () => {
    try {
      const notes = await invoke<string[]>("get_notes", { search: value.value })
      const objects: SearchResult[] = notes
        .map(name => ({
          action: "open",
          value: name,
          label: name
        }))

      const inputText = value.value.trim()
      if (inputText === "" || notes.some(item => item == inputText)) {
        items.value = objects
      } else {
        items.value = [...objects, {
          action: "create",
          value: value.value,
          label: `Create note "${value.value}"`
        }]
      }
    } catch (err) {
      emit("error", err)
    }
  }

  const onChange = async (event: any) => {
    const { originalEvent, value } = event
    if (originalEvent instanceof MouseEvent) {
      emit(value.action, value.value)
    } else if (originalEvent instanceof KeyboardEvent && originalEvent.key === "Enter") {
      emit(value.action, value.value)
    }
  }

  onMounted(() => {
    nextTick(() => {
      if (input.value) {
        input.value.$el.querySelector("input").focus()
      }
    })
  })
</script>

<template>
  <div class="card justify-center">
    <AutoComplete
      v-model="value"
      :suggestions="items"
      @change="onChange"
      @complete="onSearch"
      optionLabel="label"
      spellcheck="false"
      ref="input"
      completeOnFocus
      fluid
      placeholder="Start typing to find a note"
      />
  </div>
</template>

<style scoped>
  .p-autocomplete {
    width: 100%;
  }
</style>
