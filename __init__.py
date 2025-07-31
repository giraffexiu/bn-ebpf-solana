from binaryninja import CallingConvention, Architecture, Platform, PluginCommand
from binaryninja.typelibrary import TypeLibrary
from binaryninja.types import Type, TypeBuilder


from .ebpf import EBPF

# Register eBPF architecture first
ebpf_arch = EBPF()
ebpf_arch.register()

class DefaultCallingConvention(CallingConvention):
    name = 'Default'
    int_arg_regs = [f'r{i}' for i in range(1,10)]
    int_return_reg = 'r0'

from .solana import Solana
# Use the registered architecture by name to ensure handle is available
solana = Solana(Architecture['ebpf'])
solana.default_calling_convention = DefaultCallingConvention(Architecture['ebpf'], 'default')
solana.register('Solana')

from .solanaview import SolanaView
SolanaView.register()

# Import sidebar UI module to register LLM decompilation panel
from . import sidebar_ui

# Import batch Rust decompiler functionality
from .batch_rust_decompiler import show_batch_rust_decompiler

# Register batch Rust decompiler command
def run_batch_rust_decompiler(bv):
    """Run batch Rust decompiler"""
    show_batch_rust_decompiler(bv)

PluginCommand.register(
    "Batch Rust Decompiler (Gemini)",
    "Batch Rust decompilation using Gemini AI",
    run_batch_rust_decompiler
)
