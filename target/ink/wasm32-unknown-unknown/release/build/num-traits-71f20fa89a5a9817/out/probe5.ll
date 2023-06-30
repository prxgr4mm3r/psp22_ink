; ModuleID = 'probe5.4cee140c2822e2d8-cgu.0'
source_filename = "probe5.4cee140c2822e2d8-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

@alloc_958e480679bb0b1c2fe99415063281b8 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/5bd28f5eac1ba3569bfa8d49ec3f5acbdfdff7a0/library/core/src/num/mod.rs" }>, align 1
@alloc_2480c847e539cd47dc154768cec51300 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_958e480679bb0b1c2fe99415063281b8, [12 x i8] c"K\00\00\00w\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe5::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe55probe17hb414b1fdfb34ba17E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h08f7807bb1a7c8caE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hd6300aa9c80ac1bdE(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_2480c847e539cd47dc154768cec51300) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h08f7807bb1a7c8caE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare hidden i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17hd6300aa9c80ac1bdE(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="mvp" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="mvp" }
attributes #3 = { noreturn nounwind }
