# __floatdidf-issue

Steps to reproduce the issue:

- Clone the project
- Compile the project by running `cargo xbuild` (you need to have [`cargo-xbuild`](https://github.com/phil-opp/__floatdidf-issue/new/master?readme=1) installed).
- Disassemble the executable using `objdump`:
  
  ```
  > objdump -d target/test/debug/playground -M intel | rg -C 5 floatdidf`
  
    0000000000201020 <_ZN10playground3foo17hf4a4b11a35677989E>:
    201020:	50                   	push   rax
    201021:	48 89 3c 24          	mov    QWORD PTR [rsp],rdi
    201025:	48 8b 3c 24          	mov    rdi,QWORD PTR [rsp]
    201029:	e8 22 00 00 00       	call   201050 <__floatdidf>
    20102e:	48 bf 00 00 00 00 00 	movabs rdi,0x4014000000000000
    201035:	00 14 40 
    201038:	48 89 c6             	mov    rsi,rax
    20103b:	e8 20 00 00 00       	call   201060 <__divdf3>
    201040:	59                   	pop    rcx
  --
    20104c:	cc                   	int3   
    20104d:	cc                   	int3   
    20104e:	cc                   	int3   
    20104f:	cc                   	int3   
  
  0000000000201050 <__floatdidf>:
    201050:	50                   	push   rax
    201051:	e8 fa ff ff ff       	call   201050 <__floatdidf>
    201056:	59                   	pop    rcx
    201057:	c3                   	ret    
    201058:	cc                   	int3   
    201059:	cc                   	int3   
    20105a:	cc                   	int3  
  ```
  
  The `__floatdidf` contains endless recursion and causes a stack overflow.
