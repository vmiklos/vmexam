;
; running_light.asm
; 
; Copyright (c) 2007 by Miklos Vajna <vmiklos@frugalware.org>
; 
; This program is free software; you can redistribute it and/or modify
; it under the terms of the GNU General Public License as published by
; the Free Software Foundation; either version 2 of the License, or
; (at your option) any later version.
;
; This program is distributed in the hope that it will be useful,
; but WITHOUT ANY WARRANTY; without even the implied warranty of
; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
; GNU General Public License for more details.
;
; You should have received a copy of the GNU General Public License
; along with this program; if not, write to the Free Software
; Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, 
; USA.
;
;
;***************************************************************
;* "AVR ExperimentBoard" port assignment information:
;***************************************************************
;*
;* LED0(P):PortC.0	LED4(P):PortC.4
;* LED1(P):PortC.1	LED5(P):PortC.5
;* LED2(S):PortC.2	LED6(S):PortC.6
;* LED3(Z):PortC.3	LED7(Z):PortC.7	INT:PortE.4
;*
;* SW0:PortG.0	SW1:PortG.1	SW2:PortG.4	SW3:PortG.3
;* 
;* BT0:PortE.5	BT1:PortE.6	BT2:PortE.7	BT3:PortB.7
;*
;***************************************************************
;*
;* AIN:PortF.0	NTK:PortF.1	OPTO:PortF.2	POT:PortF.3
;*
;***************************************************************
;*
;* LCD1(VSS) = GND	LCD9(DB2): -
;* LCD2(VDD) = VCC	LCD10(DB3): -
;* LCD3(VO) = GND	LCD11(DB4): PortA.4
;* LCD4(RS) = PortA.0	LCD12(DB5): PortA.5
;* LCD5(R/W) = GND	LCD13(DB6): PortA.6
;* LCD6(E) = PortA.1	LCD14(DB7): PortA.7
;* LCD7(DB0) = -	LCD15(BLA): VCC
;* LCD8(DB1) = -	LCD16(BLK): PortB.5 (1=Backlight ON)
;*
;***************************************************************

.include "m128def.inc"	; Definition file for ATmega128 

;* Program Constants 
.equ	tconst	= 150 ; place of the timer variable in the memory

;* Program Variables Definitions 
.def	temp	= r16			; Temporary Register example 
.def	sstate	= r17			; state of the switches
.def	dir	= r18			; direction of the steps
.def	speed	= r19			; speed of the steps
.def	scarry	= r20			; carry for the steps
;*****************************************************************************
;* Reset & IT vectors
;*****************************************************************************
		
		.cseg 
		
		.org	0x0000		; Define start of Code segment 
	
		jmp		main		; Reset Handler
		jmp		dummy		; EXTINT0 Handler
		jmp		dummy		; EXTINT1 Handler
		jmp		dummy		; EXTINT2 Handler
		jmp		dummy		; EXTINT3 Handler
		jmp		int0it		; EXTINT4 Handler (INT button)
		jmp		dummy		; EXTINT5 Handler
		jmp		dummy		; EXTINT6 Handler
		jmp		dummy		; EXTINT7 Handler
		jmp		dummy		; Timer2 Compare Match Handler 
		jmp		dummy		; Timer2 Overflow Handler 
		jmp		dummy		; Timer1 Capture Event Handler 
		jmp		dummy		; Timer1 Compare Match A Handler 
		jmp		dummy		; Timer1 Compare Match B Handler 
		jmp		dummy		; Timer1 Overflow Handler 
		jmp		t0it		; Timer0 Compare Match Handler 
		jmp		dummy		; Timer0 Overflow Handler 
		jmp		dummy		; SPI Transfer Complete Handler 
		jmp		dummy		; USART0 RX Complete Handler 
		jmp		dummy		; USART0 Data Register Empty Hanlder 
		jmp		dummy		; USART0 TX Complete Handler 
		jmp		dummy		; ADC Conversion Complete Handler 
		jmp		dummy		; EEPROM Ready Hanlder 
		jmp		dummy		; Analog Comparator Handler 
		jmp		dummy		; Timer1 Compare Match C Handler 
		jmp		dummy		; Timer3 Capture Event Handler 
		jmp		dummy		; Timer3 Compare Match A Handler 
		jmp		dummy		; Timer3 Compare Match B Handler 
		jmp		dummy		; Timer3 Compare Match C Handler 
		jmp		dummy		; Timer3 Overflow Handler 
		jmp		dummy		; USART1 RX Complete Handler 
		jmp		dummy		; USART1 Data Register Empty Hanlder 
		jmp		dummy		; USART1 TX Complete Handler 
		jmp		dummy		; Two-wire Serial Interface Handler 
		jmp		dummy		; Store Program Memory Ready Handler 
	
;*************************************************************** 
;* MAIN program, Initialisation part
		.org	0x0046

;* Stack Pointer init, 
; Set stack pointer to top of RAM 

main:		ldi		temp,LOW(RAMEND)	; RAMEND = RAM vegcime
		out		SPL,temp		; (ld."m128def.inc") 
		ldi		temp,HIGH(RAMEND)
		out		SPH,temp

;< Insert your other initialisation code here> 

	; ports
	; portc 0-7: led 0-7

	ldi temp, 0b11111111 ; every bit is output
	out	DDRC, temp

	; portg 0,1,3,4: switches: 0,1,3,2

	ldi		temp,0b00000000	; every bit is input
	sts		DDRG,temp
	ldi		temp,0b11111111
	sts		PORTG,temp

	; the 4th bit of porte is the int button
	ldi		temp,0b11111111; enable everything
	out		PORTE,temp
	ldi		temp,0b00000001
	out		DDRE, temp

	; interrupt of timer 0

	ldi		temp,0b00001111
	out		TCCR0,temp			; Timer 0 TCCR0 register
	ldi		temp,108			; 11059200Hz/1024 = 108*100
	out		OCR0,temp			; Timer 0 OCR0 register
	ldi		temp,0b00000010
	out		TIMSK,temp			; Timer IT Mask register

	; interrupt of INT button
	ldi temp, 0b00000011; when the buttion is pushed down, we'll want to get an interrupt
	out EICRB, temp
	ldi temp, 0b00010000; INT can interrupt
	out EIMSK, temp
	sei			; enabling the global IT

	; init'ing vars

	ldi 	dir, 0
	ldi 	temp, 0
	out	PORTC, temp

; int the main loop, we just read the values of the switches as there is no
; interrupt for them. all the other functionality is archieved in the interrupts
loop:	lds		sstate,PING
	jmp		loop

;*****************************************************************************
;* 10 msec Timer IT rutin
;*****************************************************************************


	.dseg			 
	count:	.byte	tconst		; Timer var, allocating RAM

	.cseg

;*** timer interrupt ***
t0it:	push	temp				; saving the temp reg
	in	temp,SREG			; saving state
	push	temp

; if there is no switch on, it's turned off
		ldi 	temp, 1				; temp = 1 -> turned off
		sbrc 	sstate,3			; SW3 active ?
		ldi	temp, 0				; turn on
		sbrc 	sstate,4			; SW2 active ?
		ldi	temp, 0				; turn on
		sbrc 	sstate,1			; SW1 active ?
		ldi 	temp, 0				; turn on
		sbrc 	sstate,0			; SW0 active ?
		ldi	temp, 0				; turn on
; end of the 'turn off' check
		sbrc	temp, 0	
		jmp	t0ite				; jump if it isn't turned on

; check the counter
		lds	temp,count			; load
		dec	temp				; decrement
		sts	count,temp			; and store the timer counter
		brne t0ite				; jump if it's not too low

; time for stepping
; determining the speed
; 10...150, because it fits in the 1...15x100ms, 10ms is the base
		ldi	speed, 0			; speed=0
		ldi 	temp, 80
		sbrc 	sstate,3			; SW3 active ?
		add	speed,temp
		ldi 	temp, 40
		sbrc 	sstate,4			; SW2 active ?
		add	speed,temp
		ldi 	temp, 20
		sbrc 	sstate,1			; SW1 active ?
		add	speed,temp
		ldi 	temp, 10
		sbrc 	sstate,0			; SW0 active ?
		add	speed,temp
; storing speed
		mov	temp,speed
		sts	count,temp

; setting leds
		in	temp,PORTC			; reading the state of the LEDs
		clc					; deleting carry			

		sbrs	dir,0				; determining the direction
		jmp	t0it0				; jump if dir = 1

; step right
		lsr	temp				; dir=0: step the LEDs ahead
		brcs t0it1
		ldi	scarry, 128			; pull in a 1 by addition
		add	temp, scarry
		jmp	t0it1

; step left
t0it0:	lsl	temp					; dir=1: step the LEDs back
		brcs	t0it1				; if the 0th bit is 0, then shift in a 1
		inc	temp

t0it1:	out	PORTC,temp			; restoring LEDs

t0ite:	pop	temp				; restoring registers
		out	SREG,temp
		pop	temp

dummy:	reti

; INT button interrupt handler
int0it:	push	temp					; saving: temp reg
		push 	speed				; speed reg
		push 	sstate				; sstate reg
		in	temp,SREG			; status
		push	temp

		sbrc	dir, 0				; invert dir
		jmp	invert		
		ldi	dir, 1
		jmp	popint
invert:		ldi	dir, 0

popint:		pop	temp				; restore registers
		out	SREG,temp
		pop	sstate
		pop	speed
		pop	temp
		reti
