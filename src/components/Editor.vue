<script setup lang="ts">
  import { invoke } from "@tauri-apps/api"
  import { ref, onMounted, nextTick } from "vue"
  import Quill from "quill"

  const props = defineProps(["name", "content"])
  const emit = defineEmits(["search", "save", "error"])
  const content = ref(props.content)

  let editorInstance: Quill
  let hasChanges = false
  let busy = false
  let noteName: string

  onMounted(() => {
    nextTick(() => {
      content.value = props.content
      noteName = props.name
    })
  })

  const getNoteName = () => {
    const text = editorInstance.getText()
    // NOTE there is always at least one line in Quill
    const title = text.split('\n')[0].trim()
    if (title === "") {
      throw new Error("Please fill the first line as a name of the note")
    }
    return title
  }

  const editorLoad = ({ instance }: { instance: Quill }) => {
    editorInstance = instance
    const delta = instance.clipboard.convert({ html: props.content });
    instance.setContents(delta, 'silent');
    instance.focus()
  }

  const onChange = () => {
    hasChanges = true
  }

  const save = async () => {
    if (!hasChanges || busy) {
      return false
    }

    try {
      busy = true
      const newNoteName = getNoteName()
      await invoke("save_note", { nameOld: noteName, nameNew: newNoteName, content: content.value })
      hasChanges = false
      noteName = newNoteName

      return true
    } catch (err) {
      emit("error", err)
      return false
    } finally {
      busy = false
    }
  }
  const delayMs = 1000
  setInterval(save, delayMs)

  const handleKeyDown = async (e: KeyboardEvent) => {
    const keyCodeF = 70
    const keyCodeP = 80
    const keyCodeSemicolon = 186
    if (e.metaKey || e.ctrlKey) {
      if (
        e.keyCode === keyCodeF ||
        e.keyCode === keyCodeP ||
        e.keyCode === keyCodeSemicolon
        ) {
        if (hasChanges) {
          if (!await save()) {
            return
          }
        }
        emit("search")
      }
    }
  }
</script>

<template>
  <Editor class="editor"
    v-model="content"
    v-on:keydown="handleKeyDown($event)"
    @update:modelValue="onChange"
    @load="editorLoad"
    spellCheck=false
  />
</template>

<style scoped>
  .editor {
    width: 782px;
    height: 488px;
  }
</style>
