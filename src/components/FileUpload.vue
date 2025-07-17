<template>
  <div class="space-y-4">
    <!-- Drag & Drop Area -->
    <div 
      class="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center transition-colors duration-200"
      :class="{
        'border-blue-400 bg-blue-50': isDragOver,
        'hover:border-gray-400': !isDragOver
      }"
      @drop="handleDrop"
      @dragover="handleDragOver"
      @dragenter="handleDragEnter"
      @dragleave="handleDragLeave"
    >
      <div class="space-y-4">
        <div class="text-4xl">📁</div>
        <div>
          <p class="text-lg font-medium text-gray-700">
            Drag & drop measurement files here
          </p>
          <p class="text-sm text-gray-500">
            or click to browse files
          </p>
        </div>
        <div>
          <input
            ref="fileInput"
            type="file"
            multiple
            accept=".csv,.txt,.dat"
            class="hidden"
            @change="handleFileSelect"
          >
          <button
            @click="$refs.fileInput.click()"
            class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors duration-200"
          >
            Select Files
          </button>
        </div>
        <div class="text-xs text-gray-400">
          Supported formats: CSV, TXT, DAT
        </div>
      </div>
    </div>

    <!-- File List -->
    <div v-if="uploadedFiles.length > 0" class="space-y-2">
      <h3 class="text-lg font-medium text-gray-700">Uploaded Files:</h3>
      <div class="space-y-2">
        <div 
          v-for="(file, index) in uploadedFiles" 
          :key="index"
          class="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
        >
          <div class="flex items-center space-x-3">
            <div class="text-2xl">📄</div>
            <div>
              <div class="font-medium text-gray-700">{{ file.name }}</div>
              <div class="text-sm text-gray-500">
                {{ formatFileSize(file.size) }} • {{ file.type || 'Unknown type' }}
              </div>
            </div>
          </div>
          <button
            @click="removeFile(index)"
            class="text-red-500 hover:text-red-700 transition-colors duration-200"
            title="Remove file"
          >
            ❌
          </button>
        </div>
      </div>
    </div>

    <!-- Processing Status -->
    <div v-if="isProcessing" class="flex items-center space-x-2 text-blue-600">
      <div class="animate-spin">⏳</div>
      <span>Processing files...</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const emit = defineEmits<{
  'files-uploaded': [files: File[]]
}>()

const isDragOver = ref(false)
const uploadedFiles = ref<File[]>([])
const isProcessing = ref(false)

const handleDrop = (e: DragEvent) => {
  e.preventDefault()
  isDragOver.value = false
  
  const files = Array.from(e.dataTransfer?.files || [])
  addFiles(files)
}

const handleDragOver = (e: DragEvent) => {
  e.preventDefault()
}

const handleDragEnter = (e: DragEvent) => {
  e.preventDefault()
  isDragOver.value = true
}

const handleDragLeave = (e: DragEvent) => {
  e.preventDefault()
  // Only set to false if we're leaving the drop zone completely
  if (!e.currentTarget?.contains(e.relatedTarget as Node)) {
    isDragOver.value = false
  }
}

const handleFileSelect = (e: Event) => {
  const target = e.target as HTMLInputElement
  const files = Array.from(target.files || [])
  addFiles(files)
}

const addFiles = async (files: File[]) => {
  console.log('📁 Processing files:', files.map(f => `${f.name} (${f.type})`))
  
  const validFiles = files.filter(file => {
    const isValid = file.name.endsWith('.csv') || 
                   file.name.endsWith('.txt') || 
                   file.name.endsWith('.dat')
    
    console.log(`File ${file.name}: ${isValid ? '✅ Valid' : '❌ Invalid'}`)
    return isValid
  })
  
  if (validFiles.length === 0) {
    const fileTypes = files.map(f => f.name.split('.').pop()).join(', ')
    alert(`❌ Unsupported file types: ${fileTypes}\n\n✅ Supported formats: .csv, .txt, .dat\n\nMake sure your files have the correct extension!`)
    return
  }

  if (validFiles.length < files.length) {
    const invalidFiles = files.filter(f => !validFiles.includes(f))
    console.warn('⚠️ Some files were skipped:', invalidFiles.map(f => f.name))
    alert(`⚠️ ${invalidFiles.length} file(s) skipped (unsupported format).\n${validFiles.length} valid file(s) will be processed.`)
  }

  isProcessing.value = true
  
  uploadedFiles.value.push(...validFiles)
  emit('files-uploaded', validFiles)
  
  // Simulate processing time
  setTimeout(() => {
    isProcessing.value = false
    console.log('✅ File processing complete')
  }, 1000)
}

const removeFile = (index: number) => {
  uploadedFiles.value.splice(index, 1)
}

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes'
  
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}
</script>
