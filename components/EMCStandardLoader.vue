<!-- // components/EMCStandardLoader.vue -->
<template>
  <div class="bg-white rounded-xl p-6 shadow-lg">
    <h3 class="text-xl font-bold mb-4 text-gray-800">🔬 EMC Standard Selection</h3>
    
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">
          Select Standard:
        </label>
        <select 
          v-model="selectedStandard" 
          @change="onStandardChange"
          class="w-full p-3 border-2 border-gray-300 rounded-lg focus:border-blue-500 focus:outline-none transition-colors"
        >
          <option value="">Choose a standard...</option>
          <option value="CISPR22">CISPR 22</option>
          <option value="EN55032">EN 55032</option>
          <option value="IEC61800_3">IEC 61800-3</option>
        </select>
      </div>

      <div v-if="showClassInput" class="space-y-2">
        <label class="block text-sm font-medium text-gray-700">EMC Class:</label>
        <select 
          v-model="emcClass"
          class="w-full p-3 border-2 border-gray-300 rounded-lg focus:border-blue-500 focus:outline-none transition-colors"
        >
          <option value="A">Class A</option>
          <option value="B">Class B</option>
          <option v-if="selectedStandard === 'IEC61800_3'" value="C1">Class C1</option>
          <option v-if="selectedStandard === 'IEC61800_3'" value="C2">Class C2</option>
        </select>
      </div>

      <div v-if="showInterfaceInput" class="space-y-2">
        <label class="block text-sm font-medium text-gray-700">Interface:</label>
        <select 
          v-model="interfaceType"
          class="w-full p-3 border-2 border-gray-300 rounded-lg focus:border-blue-500 focus:outline-none transition-colors"
        >
          <option value="AC">AC</option>
          <option value="DC">DC</option>
        </select>
      </div>

      <button 
        @click="loadStandard"
        :disabled="!selectedStandard || emcStore.isLoading"
        class="w-full bg-gradient-to-r from-blue-500 to-purple-600 text-white py-3 px-6 rounded-lg font-medium hover:from-blue-600 hover:to-purple-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 transform hover:scale-105"
      >
        <span v-if="emcStore.isLoading">Loading...</span>
        <span v-else>Load Standard</span>
      </button>
    </div>

    <div v-if="emcStore.error" class="mt-4 p-4 bg-red-50 border border-red-200 text-red-700 rounded-lg">
      {{ emcStore.error }}
    </div>

    <div v-if="emcStore.currentStandard" class="mt-4 p-4 bg-green-50 border border-green-200 text-green-700 rounded-lg">
      ✅ Standard "{{ emcStore.currentStandard.name }}" loaded successfully
    </div>
  </div>
</template>

<script setup lang="ts">
const emcStore = useEMCStore()

const selectedStandard = ref('')
const emcClass = ref('B')
const interfaceType = ref('AC')

const showClassInput = computed(() => 
  ['CISPR22', 'EN55032', 'IEC61800_3'].includes(selectedStandard.value)
)

const showInterfaceInput = computed(() => 
  selectedStandard.value === 'IEC61800_3'
)

const onStandardChange = () => {
  emcClass.value = selectedStandard.value === 'IEC61800_3' ? 'C1' : 'B'
  interfaceType.value = 'AC'
}

const loadStandard = async () => {
  if (!selectedStandard.value) return

  try {
    await emcStore.loadStandard(
      selectedStandard.value, 
      emcClass.value, 
      showInterfaceInput.value ? interfaceType.value : undefined
    )
  } catch (error) {
    console.error('Failed to load standard:', error)
  }
}
</script>

